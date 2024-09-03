use std::error::Error;
use chrono;
use colored::*;
use redis::Commands;
use sayaka::{debug_fn, debug_var};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::process::Command;
use uuid::Uuid;

#[derive(Clone)]
pub struct Runner {
    perseus: PathBuf,
    con: Arc<Mutex<redis::Connection>>,
}

impl Runner {
    fn get_perseus_build(&self) -> PathBuf {
        self.perseus.join("build")
    }

    fn get_elf_path(&self, elf_name: &String) -> PathBuf {
        self.perseus.join(format!("traces/elf_test/test/{}.elf", elf_name))
    }

    fn get_zstf_path(&self, trace_name: &String) -> PathBuf {
        self.perseus.join(format!("traces/{}.zstf", trace_name))
    }

    pub fn new(perseus: &PathBuf, redis: &String) -> Result<Self, Box<dyn Error>> {
        debug_fn!(perseus,redis);
        let client = redis::Client::open(redis.as_str())?;
        let con = client.get_connection()?;
        Ok(Self {
            perseus: perseus.clone(),
            con: Arc::new(Mutex::new(con)),
        })
    }

    pub fn get_valid_workloads(&self) -> Result<Vec<[String; 2]>, Box<dyn Error>> {
        debug_fn!();
        let mut workloads = Vec::new();

        // 1. Get all .zstf files in self.perseus/traces directory
        let traces_path = self.perseus.join("traces");
        for entry in fs::read_dir(&traces_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("zstf") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    workloads.push([file_name.to_string(), "trace".to_string()]);
                }
            }
        }

        // 2. Get all .elf files in self.perseus/traces/elf_test/test directory
        let elf_test_path = self.perseus.join("traces/elf_test/test");
        for entry in fs::read_dir(&elf_test_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("elf") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    workloads.push([file_name.to_string(), "elf".to_string()]);
                }
            }
        }
        Ok(workloads)
    }

    async fn add_task_handler(&self, arch: String, uuid: Uuid, workload: String, workload_type: String) {
        debug_fn!(arch,uuid,workload,workload_type);

        let perseus_build = self.get_perseus_build();
        let perseus_bin = perseus_build.join("model");
        let workload_path;
        let command_str;
        if workload_type == "trace" {
            workload_path = self.get_zstf_path(&workload).to_str().unwrap().to_string();
            command_str = format!(
                "{} --run --arch {} --workload {} > /tmp/kamura/{}.log 2>&1",
                perseus_bin.to_str().unwrap(), arch, workload_path, uuid.to_string()
            );
        } else {
            workload_path = self.get_elf_path(&workload).to_str().unwrap().to_string();
            command_str = format!(
                "{} --run --arch {} --elf {} > /tmp/kamura/{}.log 2>&1",
                perseus_bin.to_str().unwrap(), arch, workload_path, uuid.to_string()
            );
        }
        debug_var!(command_str);
        let _: () = self.con.lock().unwrap().set(format!("KAMURA_TASK_{}", uuid).as_str(), "Running").unwrap();
        let status = Command::new("sh")
            .arg("-c")
            .arg(command_str)
            .current_dir(self.perseus.join("build"))
            .spawn()
            .expect("Failed to start the process")
            .wait()
            .await
            .expect("Process didn't complete successfully");

        if status.success() {
            let _: () = self.con.lock().unwrap().set(format!("KAMURA_TASK_{}", uuid).as_str(), "Succeed").unwrap();
        } else {
            let _: () = self.con.lock().unwrap().set(format!("KAMURA_TASK_{}", uuid).as_str(), "Failed").unwrap();
        }
    }

    pub async fn add_task(&mut self, arch: &String, workload: &String, workload_type: &String) -> Result<Uuid, Box<dyn Error>> {
        debug_fn!(arch,workload);
        let uuid = Uuid::new_v4();
        let runner = Arc::new(self.clone());
        let arch = arch.clone();
        let workload = workload.clone();
        let workload_type = workload_type.clone();

        tokio::task::spawn({
            let runner = runner.clone(); // Capture the Arc for the async task
            async move {
                runner.add_task_handler(arch, uuid, workload, workload_type).await;
            }
        });

        self.con.lock().unwrap().sadd("KAMURA_TASKS", uuid.to_string().as_str())?;
        Ok(uuid)
    }

    pub fn get_task_log(&self, uuid: &String) -> Result<String, Box<dyn Error>> {
        debug_fn!(uuid);
        let contents = fs::read_to_string(format!("/tmp/kamura/{}.log", uuid))?;
        Ok(contents)
    }

    pub fn get_task_status(&self, uuid: &String) -> redis::RedisResult<String> {
        // debug_fn!(uuid);
        self.con.lock().unwrap().get(format!("KAMURA_TASK_{}", uuid))
    }

    pub fn get_all_tasks(&mut self) -> redis::RedisResult<Vec<String>> {
        // debug_fn!();
        self.con.lock().unwrap().smembers("KAMURA_TASKS")
    }

    pub fn flush_all(&mut self) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        Ok(redis::cmd("FLUSHALL").exec(&mut self.con.lock().unwrap())?)
    }
}