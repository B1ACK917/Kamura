use redis::Connection;
use sayaka::debug_fn;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize)]
pub struct Topology {
    pub instances: HashMap<String, Vec<String>>,
    pub binding: HashMap<String, String>,
}

impl Topology {
    pub fn new() -> Self {
        Topology {
            instances: Default::default(),
            binding: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicPorts {
    pub out_port: Vec<String>,
    pub in_port: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicUnit {
    pub ports: BasicPorts,
}

pub type Units = HashMap<String, BasicUnit>;

#[derive(Clone)]
pub struct Operator {
    perseus: PathBuf,
    con: Arc<Mutex<Connection>>,
}

impl Operator {
    pub fn new(perseus: &PathBuf, redis: &String) -> Result<Self, Box<dyn Error>> {
        debug_fn!(perseus,redis);
        let client = redis::Client::open(redis.as_str())?;
        let con = client.get_connection()?;
        Ok(Self {
            perseus: perseus.clone(),
            con: Arc::new(Mutex::new(con)),
        })
    }

    pub fn list_arches(&self) -> Result<Vec<String>, Box<dyn Error>> {
        debug_fn!();
        let arch_path = self.perseus.join("arch_temp");
        if !arch_path.exists() || !arch_path.is_dir() {
            return Err(format!("The path {:?} does not exist or is not a directory", arch_path).into());
        }
        let mut arches = Vec::new();
        for entry in fs::read_dir(arch_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name() {
                    if let Some(dir_name_str) = dir_name.to_str() {
                        arches.push(dir_name_str.to_string());
                    }
                }
            }
        }
        Ok(arches)
    }

    pub fn read_arch(&self, target_arch: String) -> Result<(Units, Topology), Box<dyn Error>> {
        debug_fn!();
        let arch_path = self.perseus.join("arch_temp").join(target_arch);
        if !arch_path.exists() || !arch_path.is_dir() {
            return Err(format!("The path {:?} does not exist or is not a directory", arch_path).into());
        }
        let topology_path = arch_path.join("topology.json");
        let mut file = fs::File::open(&topology_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let topology: Topology = serde_json::from_str(&content)?;


        let units_path = arch_path.join("units.json");
        file = fs::File::open(&units_path)?;
        content.clear();
        file.read_to_string(&mut content)?;
        let units: Units = serde_json::from_str(&content)?;

        Ok((units, topology))
    }

    pub fn flush_all(&mut self) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        Ok(redis::cmd("FLUSHALL").exec(&mut self.con.lock().unwrap())?)
    }
}