mod utils;

use crate::utils::convert_to_cy_elements;
use kamura_core::consts::{OPERATOR_ARCH_DIR, OPERATOR_ARCH_LAYOUTS_SET_NAME};
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
pub struct Edge {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Topology {
    pub hierarchy: Value,
    pub instances: Value,
    pub binding: Vec<Edge>,
}

impl Topology {
    pub fn new() -> Topology {
        Topology { hierarchy: Default::default(), instances: Default::default(), binding: vec![] }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicUnit {
    pub ports: HashMap<String, Value>,
    pub params: HashMap<String, Value>,
    pub hierarchy: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BasicHierarchy {
    pub info: Vec<String>,
    pub core0: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Units {
    pub units: HashMap<String, BasicUnit>,
    pub hierarchy: BasicHierarchy,
}

impl Units {
    pub fn new() -> Units {
        Units { units: Default::default(), hierarchy: BasicHierarchy { info: vec![], core0: Default::default() } }
    }
}

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
        let arch_path = self.perseus.join(OPERATOR_ARCH_DIR);
        if !arch_path.exists() || !arch_path.is_dir() {
            return Err(format!("The path {:?} does not exist or is not a directory", arch_path).into());
        }
        let mut arches = Vec::new();
        for entry in fs::read_dir(arch_path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                arches.push(path.file_stem().unwrap().to_str().unwrap().to_string());
            }
        }
        arches.sort();
        Ok(arches)
    }

    pub fn read_units(&self) -> Result<Units, Box<dyn Error>> {
        debug_fn!();
        let units_path = self.perseus.join("config/py_file/units.json");
        let mut file = fs::File::open(&units_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let units: Units = serde_json::from_str(&content)?;

        Ok(units)
    }

    pub fn read_arch(&self, target_arch: &String) -> Result<Topology, Box<dyn Error>> {
        debug_fn!();
        let arch_path = self.perseus.join(OPERATOR_ARCH_DIR).join(format!("{target_arch}.json"));
        if !arch_path.exists() {
            return Err(format!("The path {:?} does not exist", arch_path).into());
        }
        let topology_path = arch_path;
        let mut file = fs::File::open(&topology_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let topology: Topology = serde_json::from_str(&content)?;

        Ok(topology)
    }

    pub fn parse_arch(&self, target_arch: String, reset_elements: bool) -> Result<(Topology, Vec<Value>), Box<dyn Error>> {
        debug_fn!();
        let topology = self.read_arch(&target_arch)?;
        let units = self.read_units()?;
        let elements;
        if reset_elements {
            elements = convert_to_cy_elements(&units, &topology)?;
        } else {
            let fetched: RedisResult<String> = self.con.lock().unwrap().hget(OPERATOR_ARCH_LAYOUTS_SET_NAME, format!("{target_arch}"));
            if fetched.is_err() {
                elements = convert_to_cy_elements(&units, &topology)?;
            } else {
                elements = serde_json::from_str(&fetched.unwrap()).unwrap();
            }
        }
        Ok((topology, elements))
    }

    pub fn save_arch(&self, arch_name: String, topology: Topology, elements: Vec<Value>) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        let serialized_data = serde_json::to_string(&elements).unwrap();

        let arch_path = self.perseus.join(OPERATOR_ARCH_DIR).join(format!("{arch_name}.json"));
        let topology_file = File::create(arch_path)?;
        serde_json::to_writer_pretty(topology_file, &topology)?;

        let _: () = self.con.lock().unwrap().hset(OPERATOR_ARCH_LAYOUTS_SET_NAME, format!("{arch_name}"), serialized_data)?;
        Ok(())
    }

    pub fn remove_arch(&self, arch_name: String) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        let arch_path = self.perseus.join(OPERATOR_ARCH_DIR).join(&arch_name);
        fs::remove_dir_all(&arch_path)?;

        let _: () = self.con.lock().unwrap().hdel(OPERATOR_ARCH_LAYOUTS_SET_NAME, format!("{arch_name}"))?;
        Ok(())
    }
}