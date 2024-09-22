use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug)]
pub struct SemVer {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub beta: bool
}

pub fn parse_semver_from_str(src: &str) -> SemVer {
    let mut beta = src.contains("-beta");
    let mut new_src = src.replace("-beta", "");
    let split_str: Vec<&str> = new_src.split(".").collect();
    let mut major = split_str[0].parse::<i32>().expect("Couldn't parse SemVer");
    let mut minor = split_str[1].parse::<i32>().expect("Couldn't parse SemVer");
    let mut patch = split_str[2].parse::<i32>().expect("Couldn't parse SemVer");

    SemVer {
        major, minor, patch, beta
    }
}

pub fn parse_semver_from_vec(src: Vec<i32>) -> SemVer {
    SemVer {
        major: src[0],
        minor: src[1],
        patch: src[2],
        beta: false
    }
}