# Generics yaml deserializer for Rust

This is a work i progress, its working, but usage is ugly until I figure out why serde wont
deserialize a generic.

## Dep:

    [dependencies]
    generics_yaml_deserializer = { git = "https://github.com/unixunion/rust_generics_yaml_deserializer.git", branch="master" }
    

## Usage:

    use crate::generics_yaml_deserializer::{Outer, Ptr};
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
    
    // Read the YAML serializing into "SomeStruct"        
    let resultb: Outer<SomeStruct> = serde_yaml::from_str(b).unwrap();
    
    // use match to get the Owned pointer, error if not found
    match resultb.ptr {
        Ptr::Owned(e) => {assert_eq!(10, e.a);},
        Ptr::Ref(_) => {println!("error")},
        Ptr::Owned(_) => {println!("error")}
    };
    
 