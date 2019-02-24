

    #[cfg(test)]
    mod tests {

        use crate::generics_yaml_deserializer::{Outer, Ptr};
        use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExternalStructA {
            x: u32,
            y: u32,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExternalStructB {
            a: u64,
            b: u64,
        }


        #[test]
        fn deserialize() {


            let a = r#"---
    ptr:
      x: 1
      y: 2
         "#;

            let b = r#"---
    ptr:
      a: 10
      b: 2
            "#;

            let resulta: Outer<ExternalStructA> = serde_yaml::from_str(a).unwrap();
            match resulta.ptr {
                Ptr::Owned(e) => {assert_eq!(1, e.x);},
                Ptr::Ref(e) => {println!("error")},
                Ptr::Owned(_) => {println!("error")}
            };


            let resultb: Outer<ExternalStructB> = serde_yaml::from_str(b).unwrap();
            match resultb.ptr {
                Ptr::Owned(e) => {assert_eq!(10, e.a);},
                Ptr::Ref(_) => {println!("error")},
                Ptr::Owned(_) => {println!("error")}
            };
//            let resultb: Outer<ExternalStructB> = serde_yaml::from_str(b).unwrap();
//            assert_eq!(1, resultb.ptr.a);

        }

    }

    pub mod generics_yaml_deserializer {

        use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

        use std::error::Error;

        // empty holding struct which owns a owned ptr
        #[derive(Deserialize, Debug)]
        pub struct Outer<'a, T: 'a + ?Sized> {
            #[serde(bound(deserialize = "Ptr<'a, T>: Deserialize<'de>"))]
            pub ptr: Ptr<'a, T>,
        }

        #[derive(Debug)]
        pub enum Ptr<'a, T: 'a + ?Sized> {
            Ref(&'a T),
            Owned(Box<T>),
        }

        impl<'de, 'a, T: 'a + ?Sized> Deserialize<'de> for Ptr<'a, T>
            where
                Box<T>: Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer).map(Ptr::Owned)
            }
        }




//        fn readfile<T>(filename: String) -> Result<Box<T>, Box<std::error::Error>> {
//            let f = std::fs::File::open(filename)?;
//            let config_data: Outer<T> = serde_yaml::from_reader(f)?;
//            Ok(Box::new(config_data.ptr))
//        }
//
//        fn readconfig<T>(filename: String) -> Result<Box<T>, &'static str> {
//            // read the config file
//            let config_data = readfile(filename);
//            println!("{}", config_data);
//
//            match config_data {
//                Ok(e) => {
//                    Ok(Box::from(e))
//                },
//                Err(_) => {
//                    Err("nadda")
//                }
//            }

//            // match and return the object
//            let result = match config_data.ptr {
//                Ptr::Owned(e) => Ok(e),
//                Ptr::Ref(e) => Err(":("),
//                Ptr::Owned(_) => Err(":/")
//            };
//            result
//        }

    }