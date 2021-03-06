use std::string;
use rustc_serialize::{json, base64};

#[derive(Debug)]
/// All the errors we can encounter while signing/verifying tokens
/// and a couple of custom one for when the token we are trying
/// to verify is invalid
pub enum Error {
    EncodeJSON(json::EncoderError),
    DecodeBase64(base64::FromBase64Error),
    DecodeJSON(json::DecoderError),
    Utf8(string::FromUtf8Error),

    InvalidToken,
    InvalidSignature,
    WrongAlgorithmHeader
}

macro_rules! impl_from_error {
    ($f: ty, $e: expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error { $e(f) }
        }
    }
}

impl_from_error!(json::EncoderError, Error::EncodeJSON);
impl_from_error!(base64::FromBase64Error, Error::DecodeBase64);
impl_from_error!(json::DecoderError, Error::DecodeJSON);
impl_from_error!(string::FromUtf8Error, Error::Utf8);
