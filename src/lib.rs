pub mod generics;
pub mod utils;

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::generics::manifest::{deserialize_manifest_from_str, Manifest};
    use super::*;

    #[test]
    fn test() {
        let deserialized: Manifest = deserialize_manifest_from_str(
            &fs::read_to_string("./inputs/manifest.json").unwrap()
        );

        dbg!(deserialized);
    }
}
