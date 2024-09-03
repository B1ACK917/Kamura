use colored::*;
use kamura_core::consts::{ENGINE_AUTH_MD5_DIGEST, ENGINE_AUTH_MD5_NUM};
use md5::compute;
use sayaka::debug_fn;

pub fn auth(auth_str: String) -> bool {
    debug_fn!(auth_str);
    format!("{:x}", compute(&auth_str)) == ENGINE_AUTH_MD5_DIGEST && auth_str.len() == ENGINE_AUTH_MD5_NUM
}
