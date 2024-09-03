use colored::*;
use redis::Connection;
use sayaka::debug_fn;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

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
        let arch_path = self.perseus.join("temp").join("arch");
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

    // pub fn read_arches(&self, target_arch: String) {
    //
    // }

    pub fn flush_all(&mut self) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        Ok(redis::cmd("FLUSHALL").exec(&mut self.con.lock().unwrap())?)
    }
}