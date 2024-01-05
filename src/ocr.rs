use scraper::{Html, Selector};
use tesseract::plumbing::leptonica_plumbing::Pix;

use crate::{
    errors::Error,
    utils::{Rectangle, Vector2},
};

pub fn ocr_text_location(target_text: &str, image: &[u8]) -> Result<Rectangle, Error> {
    let html = tesseract::Tesseract::new(None, "eng".into())?
        .set_image_from_mem(image)?
        .set_variable("hocr_char_boxes", "1")?
        .set_rectangle(51, 46, 571, 53)
        .get_hocr_text(0)?;

    let document = Html::parse_fragment(&html);
    let word_selector = Selector::parse(".ocrx_word")?;
    let char_selector = Selector::parse(".ocrx_cinfo")?;

    let mut bbox: Option<Rectangle> = None;

    for word in document.select(&word_selector) {
        let mut bbox_list = Vec::<Rectangle>::new();
        let mut current_index: usize = 0;
        let target_index = target_text.len();
        let target_chars = target_text.chars().collect::<Vec<char>>();

        for chara in word.select(&char_selector) {
            let current_char = chara.inner_html().chars().next().unwrap();
            let target_char = target_chars[current_index];

            if current_char != target_char {
                bbox_list.clear();
                current_index = 0;
                continue;
            }

            let bbox = parse_bbox(chara.attr("title").unwrap())?;
            bbox_list.push(bbox);
            current_index += 1;
            if current_index == target_index {
                break;
            }
        }
        if current_index == target_index {
            bbox = Some(Rectangle::new(
                bbox_list[0].pos,
                bbox_list.iter().map(|b| b.size).sum(),
            ));
            break;
        }
    }

    match bbox {
        Some(b) => Ok(b),
        None => Err(Error::InternalError("chars not found in card".to_owned())),
    }
}

pub fn parse_bbox(title: &str) -> Result<Rectangle, Error> {
    let cinfo_params = title.split(' ').collect::<Vec<&str>>();
    let x: u32 = cinfo_params[1].parse()?;
    let y: u32 = cinfo_params[2].parse()?;
    let w: u32 = cinfo_params[3].parse()?;
    let h: u32 = cinfo_params[4].trim_end_matches(";").parse()?;

    Ok(Rectangle::new(Vector2::new(x, y), Vector2::new(w-x, h-y)))
}
