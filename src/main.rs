use std::io::Cursor;

use dialoguer::Input;
use errors::Error;
use image::{io::Reader, DynamicImage, GenericImage};
use scryfall::Card;

mod ocr;
mod utils;
mod errors;

#[tokio::main]
async fn main(){
    let target_text: String = Input::new()
        .with_prompt("Text")
        .interact_text()
        .unwrap();

    let image = match build_card_collage(&target_text).await {
        Ok(i) => i,
        Err(e) => Err(e).unwrap()
    };
    image.save("result.jpg").unwrap();
}

async fn build_card_collage(text: &str) -> Result<DynamicImage, Error>{
    let words = text.split(' ');
    let mut images: Vec<DynamicImage> = Vec::new(); 

    for word in words{
        let mut current_index = 0;
        let mut splice_size = 4;

        while current_index != word.len(){
            if current_index + splice_size > word.len(){
                splice_size = word.len() - current_index;
            }
            let splice = word.get(current_index..current_index+splice_size).unwrap();

            println!("Searching for \"{}\"...", splice);

            let img = match find_card_and_crop(splice).await {
                Ok(i) => i,
                Err(_) => {
                    splice_size-=1;
                    continue;
                }
            };
            images.push(img);
            current_index+=splice_size;
        }
    }

    let result = images.iter().fold(DynamicImage::new_rgba8(0, 939), |a, b| {
        let mut new_image = DynamicImage::new_rgba8(a.width()+b.width(), a.height());
        new_image.copy_from(&a, 0, 0).unwrap();
        new_image.copy_from(b, a.width(), 0).unwrap();
        new_image
    });

    Ok(result)
}

async fn find_card_and_crop(target_text: &str) -> Result<DynamicImage, Error>{

    let results = Card::search(target_text)
    .await
    .map_err(|_| Error::CardWithTextNotFound)?
    .into_inner();

    for card in results{
        let image_url = match card.image_uris.get("large") {
            Some(url) => url,
            None => continue
        };

        let image_data = reqwest::get(image_url.as_str())
        .await.unwrap().bytes().await.unwrap();

        let bbox = ocr::ocr_text_location(target_text, &image_data);

        let rect = match bbox {
            Ok(b) => b,
            Err(_) => continue
        };

        println!("Using card: {} | {}", card.name, image_url.as_str());

        let mut img = Reader::new(Cursor::new(&image_data))
            .with_guessed_format().unwrap()
            .decode().unwrap();

        let cropped = img.crop(rect.pos.x, 0, rect.size.x, img.height());

        return Ok(cropped);
    }
    return Err(Error::CardWithTextNotFound);
}