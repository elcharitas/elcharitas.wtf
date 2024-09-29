use lazy_static::lazy_static;
use ngyn::shared::server::Bytes;
use ngyn::shared::server::NgynContext;
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::io;
use std::path::PathBuf;

lazy_static! {
    static ref STATIC_FILES: Vec<FileInfo> = {
        let cwd = std::env::current_dir().unwrap();
        let path = cwd.join("public");
        let path = path.to_str().expect("Please create a 'public' directory");
        read_directory_files(&path).expect("'public' directory not found")
    };
}

#[derive(Debug)]
pub struct FileInfo {
    path: PathBuf,
    content: Vec<u8>,
}

pub fn static_files(cx: &mut NgynContext) -> Result<Bytes, Value> {
    let current_file = cx.request().uri().path().trim_start_matches("/");
    println!("Current file: {:?}", current_file);
    let file = STATIC_FILES.iter().find(|f| f.path.ends_with(current_file));

    match file {
        Some(file) => {
            println!("Serving file: {:?}", file.path);
            Ok(Bytes::from(file.content.clone()))
        }
        None => Err(json!({"error": "File not found"})),
    }
}

fn read_directory_files(dir_path: &str) -> io::Result<Vec<FileInfo>> {
    let mut result = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let content = fs::read(&path)?;

            let file_info = FileInfo { path, content };

            result.push(file_info);
        }
    }

    Ok(result)
}