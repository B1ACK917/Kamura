use crate::consts::KAMURA_UNIVERSAL_SPLITTER;

pub fn concat(inputs: Vec<&str>) -> String {
    inputs.join(KAMURA_UNIVERSAL_SPLITTER)
}

pub fn split(input: String) -> Vec<String> {
    input.split(KAMURA_UNIVERSAL_SPLITTER).map(|s| s.to_string()).collect()
}