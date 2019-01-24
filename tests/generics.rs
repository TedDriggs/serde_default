#[macro_use]
extern crate serde_default;

#[derive(Debug, SerdeDefault, PartialEq, Eq)]
pub struct MyStruct<T> {
    #[serde(default)]
    name: String,
    #[serde(default)]
    field2: T,
    #[serde(default="Vec::new")]
    field3: Vec<T>,
}

impl<T: Default> MyStruct<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

// Here we test the tuple behavior with generics, as well as ensuring that additional
// trait bounds are not lost during the compilation process.
#[derive(Debug, SerdeDefault, PartialEq, Eq)]
#[allow(dead_code)]
pub struct MyTupleStruct<'a, T, U: ::std::fmt::Debug>(
    #[serde(default)] T,
    #[serde(default)] U,
    #[serde(default)] &'a str,
);

#[test]
fn with_string() {
    assert_eq!(MyStruct::<String>::default(), MyStruct::new(""));
}

#[test]
fn with_u16() {
    assert_eq!(MyTupleStruct(0, 0, ""), MyTupleStruct::default());
}