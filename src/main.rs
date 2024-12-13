use std::env;
use std::fs;
use std::io;
use std::path::Path;
use zip::read::ZipArchive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::error::Error;
use std::io::{Read, Write};

#[derive(Serialize, Debug)]
struct JsonItem {
  id: i32,
  name: Name,
  #[allow(non_snake_case)]
  groupID: i32
}

#[derive(Deserialize, Debug, Serialize, Clone)]
struct Name {
  de: Option<String>,
  en: Option<String>,
  es: Option<String>,
  fr: Option<String>,
  ja: Option<String>,
  ko: Option<String>,
  ru: Option<String>,
  zh: Option<String>
}

#[derive(Deserialize, Debug)]
struct Item {
  name: Name,
  #[allow(non_snake_case)]
  groupID: i32,
  #[allow(non_snake_case)]
  marketGroupID: Option<i32>
}

#[derive(Deserialize, Debug)]
struct Yaml {
  #[serde(flatten)]
  values: HashMap<String, Item>
}

fn main() -> () {
  let base_dir = env::current_dir().unwrap();
  let zip_file_path = base_dir.join("sde.zip");
  let out_file_path = base_dir.join("types.json.gz");
  let yaml_content = get_zip_content(zip_file_path.as_path()).unwrap();
  let json_content = yaml_fmt(&yaml_content).unwrap();
  let json_string = serde_json::to_string(&json_content).unwrap();
  zip(&json_string, out_file_path.as_path()).unwrap();
}


fn get_zip_content(file: &Path) -> Result<Yaml, Box<dyn Error>> {
  let mut archive = ZipArchive::new(fs::File::open(file)?)?;
  let mut file = archive.by_name("fsd/types.yaml")?;
  let mut buffer = String::new();
  if file.is_file() {
    file.read_to_string(&mut buffer)?;
  } else {
    panic!("fsd/types.yaml not found");
  };
  parse_yaml(&buffer)
}

fn parse_yaml(content: &str) -> Result<Yaml, Box<dyn Error>> {
  Ok(serde_yaml::from_str(content)?)
}

fn yaml_fmt(yaml: &Yaml) -> Result<Vec<JsonItem>, Box<dyn Error>> {
  let mut items: Vec<JsonItem> = Vec::new();
  for (key, item) in &yaml.values {
    if item.marketGroupID.is_some() {
      let json_item = JsonItem {
        id: key.parse()?,
        name: item.name.clone(),
        groupID: item.groupID
      };
      items.push(json_item);
    }
  }
  Ok(items)
}

fn zip(content: &str, out_file_path: &Path) -> io::Result<()> {
  let output_file = fs::File::create(out_file_path)?;
  let mut encoder = GzEncoder::new(output_file, Compression::default());
  encoder.write_all(content.as_bytes())?;
  let _ = encoder.finish()?;
  Ok(())
}
