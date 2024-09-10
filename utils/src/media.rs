use std::fs;
use std::path::{Path, PathBuf};

const BLOG_DIR: &str = ".media/blog";
const IMAGE_DIR: &str = ".media/image";

#[derive(Debug)]
pub enum Dir {
    BlogDir,
    ImageDir,
}

impl Dir {
    pub fn dir(&self) -> PathBuf {
        match self {
            Self::BlogDir => { Path::new(BLOG_DIR).to_path_buf() }
            Self::ImageDir => { Path::new(IMAGE_DIR).to_path_buf() }
        }
    }
}

pub fn create_folder() {
    // Create Blog Folder
    if Dir::BlogDir.dir().exists() && Dir::BlogDir.dir().is_dir() {
        println!("The blog folder already exists");
    } else {
        match fs::create_dir_all(Dir::BlogDir.dir()) {
            Ok(_) => { println!("Successfully created blog folder"); }
            Err(e) => { println!("Error creating blog folder: {}", e); }
        }
    }
    // Create Image Folder
    if Dir::ImageDir.dir().exists() && Dir::ImageDir.dir().is_dir() {
        println!("The image folder already exists");
    } else {
        match fs::create_dir_all(Dir::ImageDir.dir()) {
            Ok(_) => { println!("Successfully created image folder"); }
            Err(e) => { println!("Error creating image folder: {}", e); }
        }
    }
}
