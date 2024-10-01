use lazy_static::lazy_static;
use ngyn::shared::server::Bytes;
use ngyn::shared::server::NgynContext;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::components::error::Error;
use crate::components::AppLayout;
use crate::components::LayoutProps;
use crate::shared::*;

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

pub fn static_files(cx: &mut NgynContext) -> Result<Bytes, String> {
    let current_file = cx.request().uri().path().trim_start_matches("/");
    println!("Current file: {:?}", current_file);
    let file = STATIC_FILES.iter().find(|f| f.path.ends_with(current_file));

    match file {
        Some(file) => {
            println!("Serving file: {:?}", file.path);
            Ok(Bytes::from(file.content.clone()))
        }
        None => Err(AppLayout::with(LayoutProps {
            title: "404 Not Found".to_string(),
            children: Rsx(hypertext::rsx! {
                {Error::with()}
            }),
        })
        .render()
        .as_inner()
        .clone()),
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
        } else if path.is_dir() {
            let dir_files = read_directory_files(&path.to_str().unwrap())?;
            result.extend(dir_files);
        }
    }

    Ok(result)
}
