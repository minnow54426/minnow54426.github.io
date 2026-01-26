use crate::{Result, SerializationError};
use bincode::{deserialize, serialize};

pub fn serialize_proof<T: serde::Serialize>(proof: &T) -> Result<Vec<u8>> {
    serialize(proof).map_err(|_e| SerializationError::DeserializationFailed.into())
}

pub fn deserialize_proof<T: for<'de> serde::Deserialize<'de>>(bytes: &[u8]) -> Result<T> {
    deserialize(bytes).map_err(|_e| SerializationError::DeserializationFailed.into())
}

pub fn serialize_pk<T: serde::Serialize>(pk: &T) -> Result<Vec<u8>> {
    serialize(pk).map_err(|_e| SerializationError::DeserializationFailed.into())
}

pub fn deserialize_pk<T: for<'de> serde::Deserialize<'de>>(bytes: &[u8]) -> Result<T> {
    deserialize(bytes).map_err(|_e| SerializationError::DeserializationFailed.into())
}
