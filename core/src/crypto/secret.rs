use bincode::de::read::Reader;
use bincode::de::{BorrowDecoder, Decoder};
use bincode::enc::write::Writer;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{BorrowDecode, Decode, Encode};
use std::fmt::{Debug, Formatter};
use zeroize::{Zeroize, Zeroizing};

#[derive(Clone, PartialEq, Eq, Zeroize)]
#[zeroize(drop)]
pub struct Secret(Zeroizing<String>);

impl Secret {
    pub fn new(secret: String) -> Self {
        Self(Zeroizing::new(secret))
    }

    pub fn reveal_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn reveal_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Debug for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Secret([REDACTED])")
    }
}

impl Encode for Secret {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let length = self.reveal_bytes().len() as u64;
        let length_bytes: [u8; 8] = length.to_be_bytes();
        encoder.writer().write(&length_bytes)?;
        encoder.writer().write(self.reveal_bytes())
    }
}

impl<Context> Decode<Context> for Secret {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut length_bytes = [0u8; 8];
        decoder.reader().read(&mut length_bytes)?;

        let length = u64::from_be_bytes(length_bytes);
        let mut bytes = vec![0u8; length as usize];
        decoder.reader().read(&mut bytes)?;

        Ok(Self(Zeroizing::new(String::from_utf8(bytes).map_err(
            |err| DecodeError::Utf8 {
                inner: err.utf8_error(),
            },
        )?)))
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for Secret {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let mut length_bytes = [0u8; 8];
        decoder.reader().read(&mut length_bytes)?;

        let length = u64::from_be_bytes(length_bytes);
        let mut bytes = vec![0u8; length as usize];
        decoder.reader().read(&mut bytes)?;

        Ok(Self(Zeroizing::new(String::from_utf8(bytes).map_err(
            |err| DecodeError::Utf8 {
                inner: err.utf8_error(),
            },
        )?)))
    }
}
