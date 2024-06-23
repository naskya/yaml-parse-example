use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn read_file_as_string(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut value = String::new();
    file.read_to_string(&mut value)?;

    #[cfg(test)]
    println!("Got this string:\n{}", &value);

    Ok(value)
}

pub fn parse_with_yaml_rust(path: &str) -> Result<Vec<yaml_rust::Yaml>> {
    let yaml_string: String = read_file_as_string(path)?;
    let value: Vec<yaml_rust::Yaml> = yaml_rust::YamlLoader::load_from_str(&yaml_string)?;
    Ok(value)
}

pub fn parse_with_yaml_rust2(path: &str) -> Result<Vec<yaml_rust2::Yaml>> {
    let yaml_string: String = read_file_as_string(path)?;
    let value: Vec<yaml_rust2::Yaml> = yaml_rust2::YamlLoader::load_from_str(&yaml_string)?;
    Ok(value)
}

pub fn parse_with_serde_yaml(path: &str) -> Result<serde_yaml::Value> {
    let yaml_string: String = read_file_as_string(path)?;
    let value: serde_yaml::Value = serde_yaml::from_str(&yaml_string)?;
    Ok(value)
}

#[cfg(test)]
mod test {
    use super::{parse_with_serde_yaml, parse_with_yaml_rust, parse_with_yaml_rust2};

    const TEST_YAML_PATH: &str = "test.yaml";

    #[test]
    fn parse_sequence_using_yaml_rust() {
        let parsed_value = parse_with_yaml_rust(TEST_YAML_PATH).unwrap();
        println!("Parsed result:\n{:#?}", parsed_value);
    }

    #[test]
    fn parse_sequence_using_yaml_rust2() {
        let parsed_value = parse_with_yaml_rust2(TEST_YAML_PATH).unwrap();
        println!("Parsed result:\n{:#?}", parsed_value);
    }

    #[test]
    fn parse_sequence_using_serde_yaml() {
        let parsed_value = parse_with_serde_yaml(TEST_YAML_PATH).unwrap();
        println!("Parsed result:\n{:#?}", parsed_value);
    }
}
