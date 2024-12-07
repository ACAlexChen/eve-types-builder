use std::env;
use std::fs;
use std::io;
use std::path::Path;
use zip::read::ZipArchive;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(Serialize)]
struct JsonItem {
  id: i32,
  name: Name,
  groupID: i32
}

#[derive(Deserialize, Debug, Serialize)]
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
  groupID: i32,
  marketGroupID: Option<i32>
}

#[derive(Deserialize, Debug)]
struct Yaml {
  #[serde(flatten)]
  values: HashMap<String, Item>
}

fn main() -> () {
  let base_dir = env::current_dir().unwrap();
  let temp_dir = base_dir.join("temp");
  if !temp_dir.is_dir() {
    fs::create_dir_all(temp_dir.to_str().unwrap()).unwrap();
    let zip_file_path = base_dir.join("sde.zip");
    unzip(&zip_file_path, &temp_dir).unwrap();
  }
  let types_file_path = temp_dir.join("fsd").join("types.yaml");
  let content = parse_yaml(&types_file_path);
  println!("{:?}", content);
  let json = yaml_fmt(&content);
  let json_str = serde_json::to_string(&json);
  let json_file_path = base_dir.join("types.json");
  fs::write(&json_file_path, json_str.unwrap()).unwrap();
  zip(&json_file_path, &base_dir).unwrap();
  fs::remove_file(&json_file_path).unwrap();
  std::thread::sleep(std::time::Duration::from_secs(5));
  fs::remove_dir_all(temp_dir).unwrap();
}


fn unzip(file: &Path, out_dir: &Path) -> io::Result<()> {
  let mut archive = ZipArchive::new(fs::File::open(file)?)?;
  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let file_path = out_dir.join(file.mangled_name());
    if file.is_dir() {
      fs::create_dir_all(file_path)?;
    } else {
      if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
      }
      let mut outfile = fs::File::create(&file_path)?;
      io::copy(&mut file, &mut outfile)?;
    }
  }
  Ok(())
}

fn parse_yaml(file: &Path) -> Yaml {
  let file = fs::File::open(file).unwrap();
  let reader = io::BufReader::new(file);
  serde_yaml::from_reader(reader).unwrap()
}

fn yaml_fmt(yaml: &Yaml) -> Vec<JsonItem> {
  let mut items: Vec<JsonItem> = Vec::new();
  for (key, item) in &yaml.values {
    println!("{:?}: {:?}", key, item);
    if item.marketGroupID.is_some() {
      let json_item = JsonItem {
        id: key.parse().unwrap(),
        name: Name {
          de: item.name.de.clone(),
          en: item.name.en.clone(),
          es: item.name.es.clone(),
          fr: item.name.fr.clone(),
          ja: item.name.ja.clone(),
          ko: item.name.ko.clone(),
          ru: item.name.ru.clone(),
          zh: item.name.zh.clone()
        },
        groupID: item.groupID
      };
      items.push(json_item);
    }
  }
  items
}
fn zip(file: &Path, out_dir: &Path) -> io::Result<()> {
  let file_data = fs::File::open(file)?;
  let mut reader = io::BufReader::new(file_data);
  let output_file = fs::File::create(out_dir.join(format!("{}.gz", file.file_name().unwrap().to_str().unwrap())))?;
  let mut encoder = GzEncoder::new(output_file, Compression::default());
  io::copy(&mut reader, &mut encoder)?;
  let _ = encoder.finish()?;
  Ok(())
}
