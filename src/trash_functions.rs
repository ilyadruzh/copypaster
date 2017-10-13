extern crate same_file;
use std::path::Path;
use std::fs;

use same_file::is_same_file;

#[allow(dead_code)]
fn copy_file_to_directory(from_paths: Vec<String>) {

    for file in from_paths {
         let mut file_first = String::new();
         file_first.push_str(file.as_str());

         let second_path = Path::new(&file).file_name().unwrap().to_str().unwrap().to_string();

         println!("second_path: {}", &second_path);

         let mut file_second = String::new();
         file_second.push_str(SECONDFOLDER);
         file_second.push_str(second_path.as_str());
         
         let b = is_same_file(&file_first, &file_second).unwrap_or(false);
         println!("{} == {} ? Answer: {}",&file_first, &file_second, b);
         println!("Копируемый файл: {}", file_second);
         let _result = fs::copy(file_first, file_second);
    }
}