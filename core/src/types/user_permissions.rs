use bincode::de::read::Reader;
use bincode::de::{BorrowDecoder, Decoder};
use bincode::enc::write::Writer;
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{BorrowDecode, Decode, Encode};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct UserPermissions: i64 {
        const INVITE_MANAGER = 0b1;
        const ADMIN = 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000u64 as i64;
    }
}

impl UserPermissions {
    pub fn has_permissions(&self, permissions: UserPermissions) -> bool {
        self.contains(permissions) || self.contains(UserPermissions::ADMIN)
    }
}

impl Default for UserPermissions {
    fn default() -> Self {
        Self::empty()
    }
}

impl Encode for UserPermissions {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&self.bits().to_be_bytes())
    }
}

impl<Context> Decode<Context> for UserPermissions {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let mut bytes = [0u8; 8];
        decoder.reader().read(&mut bytes)?;
        Ok(Self::from_bits_truncate(i64::from_be_bytes(bytes)))
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for UserPermissions {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let mut bytes = [0u8; 8];
        decoder.reader().read(&mut bytes)?;
        Ok(Self::from_bits_truncate(i64::from_be_bytes(bytes)))
    }
}
