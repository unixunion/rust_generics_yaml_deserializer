

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
      a: 1
      b: 2
            "#;

            let resulta: Outer<ExternalStructA> = serde_yaml::from_str(a).unwrap();
            match resulta.ptr {
                Ptr::Owned(e) => {assert_eq!(1, e.x);},
                Ptr::Ref(e) => {println!("error")},
                Ptr::Owned(_) => {println!("error")}
            };

//            let resultb: Outer<ExternalStructB> = serde_yaml::from_str(b).unwrap();
//            assert_eq!(1, resultb.ptr.a);

        }

    }

    mod generics_yaml_deserializer {

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

//        fn readconfig<T>(config: String) -> Result<T, Box<std::error::Error>>  {
//            // read the config file
//            let file = std::fs::File::open(config)?;
//            let config_data: Outer<T> = serde_yaml::from_reader(file)?;
//            let a = match config_data.ptr {
//                Ptr::Owned(e) => Ok(e),
//                Ptr::Ref(e) => Err(":("),
//                Ptr::Owned(_) => Err(":/")
//            };
//        }

    }