use apache_avro::{AvroSchema, Reader, Writer, from_value};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Takes in a type that implements the right combination of traits and runs it through a Serde
/// round-trip and asserts the result is the same.
pub fn serde_assert<T>(obj: T)
where
    T: std::fmt::Debug + Serialize + DeserializeOwned + AvroSchema + Clone + PartialEq,
{
    assert_eq!(obj,  de(ser(obj.clone())));
}   

fn ser<T>(obj: T) -> Vec<u8>
where
    T: Serialize + AvroSchema,
{
    let schema = T::get_schema();
    let mut writer = Writer::new(&schema, Vec::new());
    writer.append_ser(obj).unwrap();
    writer.into_inner().unwrap()
}

fn de<T>(encoded: Vec<u8>) -> T
where
    T: DeserializeOwned + AvroSchema,
{
    assert!(!encoded.is_empty());
    let schema = T::get_schema();
    let mut reader = Reader::with_schema(&schema, &encoded[..]).unwrap();
    if let Some(res) = reader.next() {
        return res.and_then(|v| from_value::<T>(&v)).unwrap();
    }
    panic!("Nothing was encoded!")
}
