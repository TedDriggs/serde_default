#[macro_use]
extern crate serde_default;

#[derive(Debug, SerdeDefault, PartialEq, Eq)]
pub struct MyStruct {
    // This field is renamed to make sure serde_default is properly ignoring
    // other serde fields
    #[serde(rename = "foo", default = "field_1_default")]
    field1: u16,
    // This field is using the value from its trait default
    #[serde(default)]
    field2: String,
}

fn field_1_default() -> u16 {
    3
}

#[test]
fn check_field() {
    assert_eq!(MyStruct::default(), MyStruct { field1: 3, field2: "".into() });
}