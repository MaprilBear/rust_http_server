//! Serialization is the concept of converting a piece of structured data into a string of bytes in
//! a reversible way. This functionality allows objects to be converted to raw bytes (serialized)
//! and sent over a communication channel and then reconstructed from raw bytes (deserialized).
//!
//! For the purposes of this library, objects are serialized into a `Vec<u8>` and deserialized into
//! either `T` or `Box<T>` depending on if the object has a constant size (i.e. dynamically sized
//! types require `Box<T>` since their size cannot be known at compile-time).
//!
//! # [`Serializable`]
//! This trait enables a type to be serialized into a `Vec<u8>`. There are no restrictions on the
//! nature or composition of the type. [serialize()](`Serializable::serialize()`) borrows an object
//! and produces a clone represented as a string of bytes.
//!
//! # [`Deserializable`]
//! This trait enables a constant-sized type to be constructed from a slice.
//! [deserialize()](`Deserializable::deserialize()`) borrows a slice and returns a newly constructed
//! object.
//!
//! Deserialization can fail if the provided slice is malformed or otherwise unrepresentable as
//! the target type.
//!
//! Dynamically sized types (DSTs) cannot implement this trait. Use [`DeserializableBoxed`] instead.
//!
//! # [`DeserializableBoxed`]
//! This trait enables a constant or dynamically sized type to be constructed from a slice.
//! [deserialize_boxed()](`DeserializableBoxed::deserialize_boxed()`) borrows a slice and returns a
//! newly constructed Boxed object.
//!
//! Deserialization can fail if the provided slice is malformed or otherwise unrepresentable as
//! the target type.

////////////////////////////////////////////////////////////////////////////////////////////////////
// Serialization Trait
////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum DeserializationError {
    InvalidFormat,
    NotSupported,
}

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

/// Types that inherit from this trait have a constant size and can be deserialized into a value.
pub trait Deserializable: Serializable + Sized {
    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError>;
}

/// Types that do not have constant sizes (DSTs) must be deserialized into a Boxed object.
pub trait DeserializableBoxed: Serializable {
    fn deserialize_boxed(bytes: &[u8]) -> Result<Box<Self>, DeserializationError>;
}

/// Constant sized types may also be deserialized into a Box.
impl<T: Deserializable> DeserializableBoxed for T {
    fn deserialize_boxed(bytes: &[u8]) -> Result<Box<Self>, DeserializationError> {
        T::deserialize(bytes).map(Box::new)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// String Serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Serializable for String {
    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Deserializable for String {
    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> {
        match str::from_utf8(bytes) {
            Ok(str) => Ok(str.to_string()),
            Err(_) => Err(DeserializationError::InvalidFormat),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Vec<u8> Serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Serializable for Vec<u8> {
    fn serialize(&self) -> Vec<u8> {
        self.clone()
    }
}

impl Deserializable for Vec<u8> {
    fn deserialize(bytes: &[u8]) -> Result<Self, DeserializationError> {
        Ok(bytes.to_vec())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// &str Serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Serializable for &str {
    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Deserializable for &str {
    fn deserialize(_bytes: &[u8]) -> Result<Self, DeserializationError> {
        Err(DeserializationError::NotSupported)
    }
}
