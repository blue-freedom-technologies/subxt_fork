// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::Metadata;

use alloc::vec::Vec;

/// This trait is implemented for all types that also implement [`scale_decode::DecodeAsType`].
pub trait DecodeWithMetadata: Sized {
    /// Given some metadata and a type ID, attempt to SCALE decode the provided bytes into `Self`.
    fn decode_with_metadata(
        bytes: &mut &[u8],
        type_id: u32,
        metadata: &Metadata,
    ) -> Result<Self, scale_decode::Error>;
}

impl<T: scale_decode::DecodeAsType> DecodeWithMetadata for T {
    fn decode_with_metadata(
        bytes: &mut &[u8],
        type_id: u32,
        metadata: &Metadata,
    ) -> Result<T, scale_decode::Error> {
        let val = T::decode_as_type(bytes, type_id, metadata.types())?;
        Ok(val)
    }
}

/// This trait is implemented for all types that also implement [`scale_encode::EncodeAsType`].
pub trait EncodeWithMetadata {
    /// SCALE encode this type to bytes, possibly with the help of metadata.
    fn encode_with_metadata(
        &self,
        type_id: u32,
        metadata: &Metadata,
        bytes: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error>;
}

impl<T: scale_encode::EncodeAsType> EncodeWithMetadata for T {
    /// SCALE encode this type to bytes, possibly with the help of metadata.
    fn encode_with_metadata(
        &self,
        type_id: u32,
        metadata: &Metadata,
        bytes: &mut Vec<u8>,
    ) -> Result<(), scale_encode::Error> {
        self.encode_as_type_to(type_id, metadata.types(), bytes)?;
        Ok(())
    }
}
