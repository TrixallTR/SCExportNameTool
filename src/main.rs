use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use memchr::memmem;

fn extract(path: &Path) -> String {
    let mut exports = String::new();
    let mut bytes = Vec::new();

    let file = File::open(path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).unwrap();

    if let Some(start_index) = memmem::find(&bytes, b"START") {
        let after_start = &bytes[start_index + 5..];
        let mut current_chunk = Vec::new();
        let mut trash_size = 0;

        for &byte in after_start {
            current_chunk.push(byte);
            let chunk_length = current_chunk.len();
            let is_valid_chunk = (6..75).contains(&chunk_length) && current_chunk[chunk_length - 2..] == [0, 16];
            if is_valid_chunk {
                let valid_chunk = &current_chunk[trash_size..chunk_length - 2];
                match std::str::from_utf8(valid_chunk) {
                    Ok(valid_str) => {
                        exports.push_str(valid_str);
                        exports.push('\n');
                    },
                    Err(_) => eprintln!("Couldn't convert bytes to a valid UTF-8 string."),
                }
                if trash_size == 0 { trash_size = 16 }
                current_chunk.clear();
            }
        }        
        println!("Exports:\n{}", exports);
    }
    else { eprintln!("'START' not found in file") }

    exports
}

fn main() {
    let files = fs::read_dir("./").unwrap();
    for file in files {
        match file {
            Ok(entry) => {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();
                if file_name.ends_with(".sc") && !file_name.ends_with("_tex.sc") {
                    println!("{} \n", file_name);
                    let exports = extract(&path);
                    let output_file_name = format!("extracted_{}.txt", file_name);
                    let file = File::create(output_file_name).unwrap();
                    let mut writer = BufWriter::new(file);

                    writer.write_all(exports.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
            }
            Err(e) => eprintln!("Failed to read directory entry: {}", e)
        }
    }
}
