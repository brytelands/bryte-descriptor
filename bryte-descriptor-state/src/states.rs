pub trait SchemaEvent: borsh::BorshSerialize + borsh::BorshDeserialize + borsh::BorshSchema {
    fn generate_schema(self) -> Vec<u8>;
    fn data(&self) -> Vec<u8>;
}

pub trait SchemaEventAnchor: borsh::BorshSchema {
    fn generate_schema(self) -> Vec<u8>;
}

/// 8 byte unique identifier for a type.
pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
    fn discriminator() -> [u8; 8] {
        Self::DISCRIMINATOR
    }
}

pub trait Descriptor {
    fn schema(&self) -> Vec<u8>;
}

pub trait DescriptorSerialize {
    fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) {
        // Ok(())
    }
}

pub trait DescriptorDeserialize: Sized {
    /// Deserializes previously initialized account data. Should fail for all
    /// uninitialized accounts, where the bytes are zeroed. Implementations
    /// should be unique to a particular account type so that one can never
    /// successfully deserialize the data of one account type into another.
    /// For example, if the SPL token program were to implement this trait,
    /// it should be impossible to deserialize a `Mint` account into a token
    /// `Account`.
    fn try_deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Self::try_deserialize_unchecked(buf)
    }

    /// Deserializes account data without checking the account discriminator.
    /// This should only be used on account initialization, when the bytes of
    /// the account are zeroed.
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::io::Result<Self>;
}

#[derive(Debug, Clone)]
pub struct DescriptorError;

impl std::fmt::Display for DescriptorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An error occurred serializing the descriptor.")
    }
}

impl DescriptorError {
    pub fn new() -> Self {
        DescriptorError {}
    }
}