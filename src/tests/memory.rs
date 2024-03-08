// use bytes::Bytes;
// use serde::{Deserialize, Serialize};
// use crate::{backends::memory_backend::*, StorageBackend, StorageShard};

// #[derive(Hash, Serialize, Deserialize)]
// pub struct Person {
//     pub name: String,
//     pub age: u32,
//     pub address: Vec<String>,
// }

// fn setup() -> MemoryBackend {
//     MemoryBackend::open("").unwrap()
// }

// #[test]
// fn test_shard() -> Result<(), MemoryError> {
//     let be = setup();
//     let shard = be.open_shard("test")?;

//     let item = Person {
//         name: "Richard Digbick".to_string(),
//         age: 69,
//         address: vec![format!("420 Some Rd."), format!("City, State, 69420")],
//     };

//     let value = shard.prepare_value(&item)?;
//     let key = shard.compute_key(&value)?;
//     let _ = shard.put(key, value.clone())?;

//     let got = shard.get(key)?;
//     assert_eq!(value, got);

//     match shard.delete(key)? {
//         Some(deleted) => {
//             assert_eq!(value, deleted);
//             assert_eq!(got, deleted);
//         }
//         None => return Err(MemoryError::new("test", "delete failed")),
//     }

//     // Check we don't get a result for the same ID in a different shard
//     let s2 = be.open_shard("testing2")?;
//     let got2 = s2.get(key);
//     assert!(got2.is_err());

//     Ok(())
// }
