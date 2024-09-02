use chrono::{DateTime, Utc};
use colored::*;
use redis::Commands;
use sayaka::debug_fn;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;
use tokio::process::Command;

#[derive(Clone)]
pub struct Integrator {
    perseus: PathBuf,
    con: Arc<Mutex<redis::Connection>>,
}

impl Integrator {
    pub fn new(perseus: &PathBuf, redis: &String) -> Result<Self, Box<dyn std::error::Error>> {
        debug_fn!(perseus,redis);
        let client = redis::Client::open(redis.as_str())?;
        let con = client.get_connection()?;
        Ok(Self {
            perseus: perseus.clone(),
            con: Arc::new(Mutex::new(con)),
        })
    }

    pub fn get_perseus_path(&self) -> String {
        self.perseus.to_str().unwrap().to_string()
    }

    pub fn get_perseus_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        debug_fn!();
        let output = std::process::Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .current_dir(&self.perseus)
            .output()?;

        if output.status.success() {
            let commit_hash = String::from_utf8(output.stdout)?.trim().to_string();
            Ok(commit_hash)
        } else {
            Err(format!(
                "Failed to retrieve commit hash: {}",
                String::from_utf8_lossy(&output.stderr)
            ).into())
        }
    }

    pub fn get_build_date(&self) -> Result<String, Box<dyn std::error::Error>> {
        debug_fn!();
        let build_model_path = self.perseus.join("build/model");

        let metadata = fs::metadata(&build_model_path)?;
        let modified_time = metadata.modified()?;

        let duration_since_epoch = modified_time.duration_since(UNIX_EPOCH)?;
        let datetime = DateTime::from_timestamp(
            duration_since_epoch.as_secs() as i64,
            duration_since_epoch.subsec_nanos(),
        ).ok_or("Failed to convert timestamp")?.with_timezone(&Utc);

        let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        Ok(formatted_date)
    }

    pub fn check_valid(&self) -> bool {
        debug_fn!();
        let build_model_path = self.perseus.join("build/model");
        build_model_path.exists()
    }

    pub async fn rebuild_perseus_handler(&self) {
        debug_fn!();

        let _: () = self.con.lock().unwrap().set("KAMURA_INT_REBUILD_PERSEUS", "Running").unwrap();

        // Step 1: Enter the perseus_path
        let perseus_path = &self.perseus;

        // Step 2: Delete the build folder if it exists
        let build_path = perseus_path.join("build");
        if build_path.exists() {
            fs::remove_dir_all(&build_path).unwrap();
        }

        // Step 3: Run cmake -Bbuild
        let cmake_status = Command::new("sh")
            .arg("-c")
            .arg("cmake -Bbuild >/dev/null 2>&1")
            .current_dir(perseus_path)
            .spawn()
            .expect("Command failed to start")
            .wait()
            .await
            .expect("Command failed to run");

        if !cmake_status.success() {
            let _: () = self.con.lock().unwrap().set("KAMURA_INT_REBUILD_PERSEUS", "Failed to execute cmake -Bbuild".to_string()).unwrap();
            return;
        }

        // Step 4: Run make in the build directory
        let make_status = Command::new("sh")
            .arg("-c")
            .arg("make -j10 >/dev/null 2>&1")
            .current_dir(perseus_path.join("build"))
            .spawn()
            .expect("Command failed to start")
            .wait()
            .await
            .expect("Command failed to run");

        if !make_status.success() {
            let _: () = self.con.lock().unwrap()
                .set("KAMURA_INT_REBUILD_PERSEUS", "Failed to execute make".to_string()).unwrap();
            return;
        }

        let _: () = self.con.lock().unwrap().set("KAMURA_INT_REBUILD_PERSEUS", "Succeed").unwrap();
    }

    pub fn rebuild_perseus(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug_fn!();
        let perseus_path = &self.perseus;
        if !perseus_path.exists() {
            Err(format!("Perseus path does not exist: {:?}", perseus_path).into())
        } else {
            let integrator = Arc::new(self.clone());
            tokio::task::spawn({
                let integrator = integrator.clone();
                async move {
                    integrator.rebuild_perseus_handler().await;
                }
            });
            Ok(())
        }
    }

    pub fn get_perseus_rebuild_status(&self) -> redis::RedisResult<String> {
        debug_fn!();
        self.con.lock().unwrap().get("KAMURA_INT_REBUILD_PERSEUS")
    }
}