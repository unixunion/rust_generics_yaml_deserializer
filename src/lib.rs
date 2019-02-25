

    #[cfg(test)]
    mod tests {

        use crate::generics_yaml_deserializer::{Outer, Ptr};
        use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
        use crate::generics_yaml_deserializer::readfile;

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
    data:
      x: 1
      y: 2
         "#;

            let b = r#"---
    data:
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

//            let resultc: Box<Outer<ExternalStructA>> = readfile("ExternalStructA.yaml");

//            let resultb: Outer<ExternalStructB> = serde_yaml::from_str(b).unwrap();
//            assert_eq!(1, resultb.ptr.a);

        }

    }

    pub mod generics_yaml_deserializer {

        use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

        use std::error::Error;

        // empty holding struct which owns a owned ptr
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct Outer<'a, T: 'a + ?Sized> {
            #[serde(bound(deserialize = "Ptr<'a, T>: Deserialize<'de>"), rename = "data")]
            pub ptr: Ptr<'a, T>,
        }

        #[derive(Serialize, Debug, Clone)]
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


        // filename should be &str here
        pub fn readfile<'a, T: ?Sized>(filename: &str) -> Result<Box<Outer<'a, T>>, Box<std::error::Error>>
            where
                    for<'de> T: Deserialize<'de> + 'a
        {
            let f = std::fs::File::open(filename)?;
            let config_data: Outer<T> = serde_yaml::from_reader(f)?;
            Ok(Box::new(config_data))
        }

        // filename should be &str here
        pub fn readconfig<'a, T: ?Sized>(filename: &str) -> Result<Box<Outer<'a, T>>, &'static str>
            where
                    for<'de> T: Deserialize<'de> + 'a
        {
            // read the config file
            let config_data = readfile(filename);
            match config_data {
                Ok(e) => {
                    Ok(Box::new(*e)) // need to deref the Box before reboxing
                },
                Err(_) => {
                    Err("nadda")
                }
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