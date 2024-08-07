#[macro_use]
extern crate serde_default;

#[derive(Debug, DefaultFromSerde, PartialEq, Eq)]
pub struct MyStruct {
    #[serde(default = "field_1_default")]
    field1: u16,
    #[serde(default)]
    field2: String,
}

fn field_1_default() -> u16 {
    3
}

#[derive(Debug, DefaultFromSerde)]
#[allow(dead_code)]
pub struct MyTupleStruct(
    #[serde(default = "field_1_default")] u16,
    #[serde(default)] String,
);

fn main() {
    println!("{:?}", MyStruct::default());
}
