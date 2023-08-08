// anyhow::Result
use anyhow::Result;
use serde_json::Value;

fn main() -> Result<()> {
    let config = std::fs::read_to_string("cluster.json")?; // std::io::Error
    let map: Value = serde_json::from_str(&config)?; // serde_json::Error
    println!("cluster info: {:#?}", map);
    Ok(())
}


// context
// use anyhow::{Context, Result};
// use std::fs;
// use std::path::PathBuf;

// pub struct ImportantThing {
//     path: PathBuf,
// }

// pub fn do_it(it: ImportantThing) -> Result<Vec<u8>> {
//     let path = &it.path;
//     let content = fs::read(path)
//         .with_context(|| format!("Failed to read instrs from {}", path.display()))?;

//     Ok(content)
// }

// fn main() -> Result<()> {
//     let it = ImportantThing {
//         path: PathBuf::from("instructions.txt"),
//     };
//     let content = do_it(it)?;
//     println!("{:?}", content);
//     Ok(())
// }

// anyhow!
// use anyhow::{bail, Result};

// fn validate(key: &str) -> Result<()> {
//     if key.len() != 16 {
//         bail!("key length must be 16 characters, got {:?}", key);
//     }

//     Ok(())
// }

// fn main() {
//     let key = "1234567890";
//     let err = validate(key).unwrap_err();
//     println!("Validation Error: {}", err);
// }

// use anyhow::{Context, Result, bail};
// use serde_json::Value;

// fn get_cluster_info() -> Result<Value> {
//     let config = std::fs::read_to_string("cluster.json")?;
//     let map: Value = serde_json::from_str(&config)?;
//     Ok(map)
// }

// fn validate_cluster_map(cluster_map: &Value) -> Result<()> {
//     // cluster_map.get("nodes").context("Cluster name is missing")?;
//     if cluster_map.get("nodes").is_none() {
//         // return Err(anyhow::anyhow!("Cluster nodes are missing"));
//         bail!("Cluster nodes are missing");
//     }
//     Ok(())
// }

// fn main() -> Result<()> {
//     let cluster_map = get_cluster_info()?;
//     validate_cluster_map(&cluster_map)?;
//     println!("{:?}", cluster_map);
//     Ok(())
// }

// usage anyhow
// use anyhow::{Context, Result};
// use std::fs;
// use std::path::PathBuf;

// pub struct ImportantThing {
//     path: PathBuf,
// }

// impl ImportantThing {
//     pub fn detach(&mut self) -> Result<()> {
//         Ok(())
//     }
// }

// pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
//     it.detach().context("Failed to detach the important thing")?;

//     let path = &it.path;
//     let content = fs::read(path)
//         .with_context(|| format!("Failed to read instrs from {}", path.display()))?;

//     Ok(content)
// }

// fn main() -> Result<()> {
//     let it = ImportantThing {
//         path: PathBuf::from("instructions.txt"),
//     };
//     let content = do_it(it)?;
//     println!("{:?}", content);
//     Ok(())
// }
