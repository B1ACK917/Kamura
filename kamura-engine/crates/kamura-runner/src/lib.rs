use chrono::{Local, NaiveDateTime};
use kamura_core::consts::{KAMURA_UNIVERSAL_TIMESTAMP_FMT, OPERATOR_ARCH_DIR, RUNNER_ELF_PATH, RUNNER_OTHER_WORKLOAD_PATH, RUNNER_TASKS_SET_NAME, RUNNER_ZSTF_PATH};
use kamura_core::func::concat;
use kamura_core::split;
use redis::{Commands, RedisResult};
use sayaka::{debug_fn, debug_var};
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::{fs, io};
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

    fn get_workload_path(&self, workload_name: &String, workload_type: &String) -> PathBuf {
        if workload_type == "trace" {
            self.perseus.join(RUNNER_ZSTF_PATH).join(format!("{workload_name}.zstf"))
        } else if workload_type == "elf" {
            self.perseus.join(RUNNER_ELF_PATH).join(format!("{workload_name}.elf"))
        } else if workload_type == "else" {
            PathBuf::from(RUNNER_OTHER_WORKLOAD_PATH).join(format!("{workload_name}.zstf"))
        } else {
            PathBuf::new()
        }
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
        let traces_path = self.perseus.join(RUNNER_ZSTF_PATH);
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
        let elf_test_path = self.perseus.join(RUNNER_ELF_PATH);
        for entry in fs::read_dir(&elf_test_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("elf") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    workloads.push([file_name.to_string(), "elf".to_string()]);
                }
            }
        }

        // 3. Get all other files in share
        let other_path = PathBuf::from(RUNNER_OTHER_WORKLOAD_PATH);
        for entry in fs::read_dir(&other_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("zstf") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    workloads.push([file_name.to_string(), "else".to_string()]);
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
        let arch_path = self.perseus.join(OPERATOR_ARCH_DIR).join(format!("{arch}.json")).to_str().unwrap().to_string();
        fs::create_dir_all("/tmp/kamura").unwrap();
        if workload_type == "trace" || workload_type == "else" {
            workload_path = self.get_workload_path(&workload, &workload_type).to_str().unwrap().to_string();
            command_str = format!(
                "{} --json {} --workload {} > /tmp/kamura/{}.log 2>&1",
                perseus_bin.to_str().unwrap(), arch_path, workload_path, uuid.to_string()
            );
        } else {
            workload_path = self.get_workload_path(&workload, &workload_type).to_str().unwrap().to_string();
            command_str = format!(
                "{} --arch {} --elf {} > /tmp/kamura/{}.log 2>&1",
                perseus_bin.to_str().unwrap(), arch_path, workload_path, uuid.to_string()
            );
        }
        debug_var!(command_str);
        let status = Command::new("sh")
            .arg("-c")
            .arg(command_str)
            .current_dir(self.perseus.join("build"))
            .spawn()
            .expect("Failed to start the process")
            .wait()
            .await
            .expect("Process didn't complete successfully");

        let old_status: String = self.con.lock().unwrap().hget(RUNNER_TASKS_SET_NAME, format!("{uuid}")).unwrap();
        let now = Local::now();
        let timestamp = now.format(KAMURA_UNIVERSAL_TIMESTAMP_FMT).to_string();
        let new_status;
        if status.success() {
            new_status = old_status.replace("Running", "Succeed");
        } else {
            new_status = old_status.replace("Running", "Failed");
        }
        let _: () = self.con.lock().unwrap().hset(RUNNER_TASKS_SET_NAME, format!("{uuid}"), concat(vec![new_status.as_str(), &timestamp])).unwrap();
    }

    pub async fn add_task(&mut self, arch: &String, workload: &String, workload_type: &String) -> Result<Uuid, Box<dyn Error>> {
        debug_fn!(arch,workload);
        let uuid = Uuid::new_v4();
        let runner = Arc::new(self.clone());
        let arch_ = arch.clone();
        let workload_ = workload.clone();
        let workload_type = workload_type.clone();

        tokio::task::spawn({
            let runner = runner.clone(); // Capture the Arc for the async task
            async move {
                runner.add_task_handler(arch_, uuid, workload_, workload_type).await;
            }
        });

        let now = Local::now();
        let timestamp = now.format(KAMURA_UNIVERSAL_TIMESTAMP_FMT).to_string();
        self.con.lock().unwrap().hset(RUNNER_TASKS_SET_NAME, uuid.to_string().as_str(), concat(Vec::from(["Running", arch, workload, &timestamp])))?;
        Ok(uuid)
    }

    pub fn get_task_log(&self, uuid: &String) -> io::Result<String> {
        // debug_fn!(uuid);
        let contents = fs::read_to_string(format!("/tmp/kamura/{}.log", uuid))?;
        Ok(contents)
    }

    pub fn get_task_info(&self, uuid: &String) -> RedisResult<[String; 5]> {
        debug_fn!(uuid);
        let status: String = self.con.lock().unwrap().hget(RUNNER_TASKS_SET_NAME, format!("{uuid}"))?;
        let parts = split(status);
        if parts.len() < 5 {
            Ok([parts[1].clone(), parts[2].clone(), parts[3].clone(), "None".to_string(), "None".to_string()])
        } else {
            let time1 = NaiveDateTime::parse_from_str(parts[3].as_str(), KAMURA_UNIVERSAL_TIMESTAMP_FMT)
                .expect("Failed to parse timestamp1");
            let time2 = NaiveDateTime::parse_from_str(parts[4].as_str(), KAMURA_UNIVERSAL_TIMESTAMP_FMT)
                .expect("Failed to parse timestamp2");
            let duration = time2 - time1;
            Ok([parts[1].clone(), parts[2].clone(), parts[3].clone(), parts[4].clone(), format!("{}s", duration.num_seconds().to_string())])
        }
    }

    pub fn get_task_status(&self, uuid: &String) -> RedisResult<String> {
        // debug_fn!(uuid);
        let status: String = self.con.lock().unwrap().hget(RUNNER_TASKS_SET_NAME, format!("{uuid}"))?;
        let parts = split(status);
        Ok(parts[0].clone())
    }

    pub fn get_all_tasks(&mut self) -> RedisResult<Vec<String>> {
        // debug_fn!();
        self.con.lock().unwrap().hkeys(RUNNER_TASKS_SET_NAME)
    }

    pub fn remove_task(&mut self, uuid: &String) -> Result<(), Box<dyn Error>> {
        let _: () = self.con.lock().unwrap().hdel(RUNNER_TASKS_SET_NAME, format!("{uuid}"))?;
        Ok(())
    }
}