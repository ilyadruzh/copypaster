extern crate walkdir;
extern crate time;
extern crate rand;
extern crate fs_extra;
extern crate same_file;

use time::PreciseTime;

use std::fs;

// Составляет массив имён файлов
pub fn search(dir: String) -> Vec<String> {

    let start = PreciseTime::now();
    
    println!("Search in: {} START", &dir);
    let paths = fs::read_dir(&dir).unwrap();
    let mut list : Vec<String> = Vec::new();
    
    for path in paths {
            let pathforname = path.unwrap().path();
            //println!("pathforname: {:?}", pathforname);
            let file_name = pathforname.file_name();
            //println!("file_name: {:?}", file_name.unwrap().to_str().unwrap().to_string());
            list.push(file_name.unwrap().to_str().unwrap().to_string());
    }
    list.sort();

    let end = PreciseTime::now();
    println!("Search in: {} END", &dir);
    println!("TIME for: {} ", start.to(end));
    return list;
}
