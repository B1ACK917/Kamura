mod utils;

use crate::utils::convert_to_cy_elements;
use redis::{Commands, Connection, RedisResult};
use sayaka::debug_fn;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize)]
pub struct Topology {
    pub instances: HashMap<String, Vec<String>>,
    pub binding: Vec<HashMap<String, String>>,
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
    pub out_ports: Vec<String>,
    pub in_ports: Vec<String>,
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
        arches.sort();
        Ok(arches)
    }

    pub fn read_arch(&self, target_arch: &String) -> Result<(Units, Topology), Box<dyn Error>> {
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

    pub fn parse_arch(&self, target_arch: String, reset_elements: bool) -> Result<(Units, Topology, Vec<Value>), Box<dyn Error>> {
        debug_fn!();
        let (units, topology) = self.read_arch(&target_arch)?;
        let elements;
        if reset_elements {
            elements = convert_to_cy_elements(&units, &topology)?;
        } else {
            let fetched: RedisResult<String> = self.con.lock().unwrap().hget("KAMURA_OP_ELEMENTS", format!("{target_arch}"));
            if fetched.is_err() {
                elements = convert_to_cy_elements(&units, &topology)?;
            } else {
                elements = serde_json::from_str(&fetched.unwrap()).unwrap();
            }
        }
        Ok((units, topology, elements))
    }

    pub fn save_arch(&self, arch_name: String, units: Units, topology: Topology, elements: Vec<Value>) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        let serialized_data = serde_json::to_string(&elements).unwrap();

        let arch_path = self.perseus.join("arch_temp").join(&arch_name);
        fs::create_dir_all(&arch_path)?;

        let units_path = arch_path.join("units.json");
        let units_file = File::create(units_path)?;
        serde_json::to_writer_pretty(units_file, &units)?;

        let topology_path = arch_path.join("topology.json");
        let topology_file = File::create(topology_path)?;
        serde_json::to_writer_pretty(topology_file, &topology)?;

        let _: () = self.con.lock().unwrap().hset("KAMURA_OP_ELEMENTS", format!("{arch_name}"), serialized_data)?;
        Ok(())
    }

    pub fn flush_arch_elements(&mut self) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        let arches: Vec<String> = self.con.lock().unwrap().hkeys("KAMURA_OP_ELEMENTS")?;
        for arch in arches {
            self.con.lock().unwrap().hdel("KAMURA_OP_ELEMENTS", format!("{arch}"))?;
        }
        Ok(())
    }

    pub fn flush_all(&mut self) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        Ok(redis::cmd("FLUSHALL").exec(&mut self.con.lock().unwrap())?)
    }
}