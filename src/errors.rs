use std::{fmt::Display, num::ParseIntError};

use scraper::error::SelectorErrorKind;
use tesseract::{plumbing::{leptonica_plumbing::PixReadMemError, TessBaseApiGetTsvTextError, TessBaseApiGetHocrTextError}, SetVariableError};


#[derive(Debug)]
pub enum Error{
    OcrInitError,
    ImageReadError,
    ImageOcrError,
    InternalError(String),
    CardWithTextNotFound
}

impl Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error{}

impl From<tesseract::InitializeError> for Error{
    fn from(_: tesseract::InitializeError) -> Self {
        Self::OcrInitError
    }
}

impl From<PixReadMemError> for Error {
    fn from(value: PixReadMemError) -> Self {
        Self::ImageReadError
    }
}

impl From<TessBaseApiGetHocrTextError> for Error{
    fn from(value: TessBaseApiGetHocrTextError) -> Self {
        Self::ImageOcrError
    }
}

impl From<SetVariableError> for Error{
    fn from(value: SetVariableError) -> Self {
        Self::OcrInitError
    }
}

impl<'a> From<SelectorErrorKind<'a>> for Error {
    fn from(value: SelectorErrorKind) -> Self {
        Self::InternalError("Error parsing css selector".to_owned())
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::InternalError("Error parsing integer".to_owned())
    }
}