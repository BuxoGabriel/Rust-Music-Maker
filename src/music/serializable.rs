
///
pub trait Serializable {
    fn serialize(&self) -> Result<Vec<u8>, &'static str>;
    fn deserialize(serialized_data: &[u8]) -> Result<Self, &'static str>
    where 
        Self: Sized;
}