use anyhow::Result;
use blake3::Hasher;
use iroh_blobs::Hash;
use minicbor::{Decode, Decoder, Encode, Encoder, decode, encode};

pub type IrohBlobAddress = Hash;

mod cbor_hash {
    use super::*;

    pub fn encode<C, W: encode::Write>(
        hash: &Hash,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), encode::Error<W::Error>> {
        e.bytes(hash.as_bytes())?;
        Ok(())
    }

    pub fn decode<'b, C>(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Hash, decode::Error> {
        let bytes = d.bytes()?;
        let array: [u8; 32] = bytes.try_into().map_err(|_| {
            decode::Error::message("invalid byte length for Hash, expected 32 bytes")
        })?;
        Ok(Hash::from(array))
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct IrohRhex {
    #[n(0)]
    #[cbor(encode_with = "cbor_hash::encode", decode_with = "cbor_hash::decode")]
    intent: IrohBlobAddress,
    #[n(1)]
    #[cbor(encode_with = "cbor_hash::encode", decode_with = "cbor_hash::decode")]
    context: IrohBlobAddress,
    #[n(2)]
    #[cbor(encode_with = "cbor_hash::encode", decode_with = "cbor_hash::decode")]
    sigs: IrohBlobAddress,
}

impl IrohRhex {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        Ok(minicbor::to_vec(self)?)
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        Ok(minicbor::decode(&data)?)
    }

    pub fn curr_hash(&self) -> Result<Hash> {
        let mut hasher = Hasher::new();
        hasher.update(&self.to_vec()?);
        Ok(Hash::from(hasher.finalize().as_bytes()))
    }

    pub fn address(&self) -> Result<[u8; 32]> {
        let mut hasher = Hasher::new();
        hasher.update(&self.to_vec()?);
        Ok(hasher.finalize().as_bytes().clone())
    }
}
