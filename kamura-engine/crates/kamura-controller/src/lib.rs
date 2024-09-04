use redis::{Commands, Connection};
use sayaka::debug_fn;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Controller {
    con: Arc<Mutex<Connection>>,
}

impl Controller {
    pub fn new(redis: &String) -> Result<Self, Box<dyn Error>> {
        debug_fn!(redis);
        let client = redis::Client::open(redis.as_str())?;
        let con = client.get_connection()?;
        Ok(Self {
            con: Arc::new(Mutex::new(con)),
        })
    }

    pub fn flush_hashset(&mut self, target: &String) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        let arches: Vec<String> = self.con.lock().unwrap().hkeys(&target)?;
        for arch in arches {
            self.con.lock().unwrap().hdel(&target, format!("{arch}"))?;
        }
        Ok(())
    }

    pub fn flush_all(&mut self) -> Result<(), Box<dyn Error>> {
        debug_fn!();
        Ok(redis::cmd("FLUSHALL").exec(&mut self.con.lock().unwrap())?)
    }
}