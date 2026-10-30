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
    fn deserialize(bytes: Vec<u8>) -> Result<Self, DeserializationError>;
}

/// Types that do not have constant sizes (DSTs) must be deserialized into a Boxed object.
pub trait DeserializableBoxed: Serializable {
    fn deserialize_boxed(bytes: Vec<u8>) -> Result<Box<Self>, DeserializationError>;
}

/// Constant sized types may also be deserialized into a Box.
impl<T: Deserializable> DeserializableBoxed for T {
    fn deserialize_boxed(bytes: Vec<u8>) -> Result<Box<Self>, DeserializationError> {
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
    fn deserialize(bytes: Vec<u8>) -> Result<Self, DeserializationError> {
        match str::from_utf8(bytes.as_slice()) {
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
    fn deserialize(bytes: Vec<u8>) -> Result<Self, DeserializationError> {
        Ok(bytes)
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
    fn deserialize(_bytes: Vec<u8>) -> Result<Self, DeserializationError> {
        Err(DeserializationError::NotSupported)
    }
}