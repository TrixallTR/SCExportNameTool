use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use memchr::memmem;

fn extract(path: &Path) -> String{
    let mut exports = String::new();
    match fs::read(path) {
        Ok(bytes) => {
            if let Some(start_index) = memmem::find(&bytes, b"START") {
                let after_start = &bytes[start_index + 5..];
                let mut current_chunk: Vec<u8> = Vec::new();
                let mut trash_size = 0;
                for &byte in after_start {
                    current_chunk.push(byte);
                    if current_chunk.len() >= 6 && current_chunk[current_chunk.len() - 2.. ] == vec![0, 16] && current_chunk.len() < 75 {
                        //println!("BBB {:?}", current_chunk);
                        if let Ok(valid_string) = String::from_utf8(current_chunk[trash_size..current_chunk.len() - 2].to_vec()) {
                            exports.push_str(&valid_string);
                            exports.push('\n');
                            //println!("{:?}", current_chunk);
                        } 
                        else {
                            //println!("AAA {:?}", current_chunk);
                            eprintln!("Couldn't convert bytes to a valid UTF-8 string.");
                        }
                        if trash_size == 0 {
                            trash_size = 16;
                        }
                        current_chunk.clear();
                    }
                }
                println!("Exports:\n{}", exports);
            }
            else {
                eprintln!("'START' not found in file");
            }
        }
        Err(e) => {
            eprintln!("Couldn't read file: {}", e);
        }
    }
    return exports
}

fn main() {
    let files = fs::read_dir("./").expect("Failed to read the directory");

    for file in files {
        match file {
            Ok(entry) => {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();
                if file_name.ends_with(".sc") && !file_name.ends_with("_tex.sc") {
                    println!("{} \n", file_name);
                    let exports = extract(&path);
                    let output_file_name = format!("extracted_{}.txt", file_name);
                    let output_file_path = Path::new(&output_file_name);
                    let file = File::create(output_file_path).expect("Could not create file");
                    let mut writer = BufWriter::new(file);

                    writer.write_all(&exports.as_bytes()).expect("Couldn't write to file.");
                    writer.flush().expect("Couldn't flush.");
                }
            }
            Err(e) => {
                eprintln!("Failed to read directory entry: {}", e);
            }
        }
    }
}
