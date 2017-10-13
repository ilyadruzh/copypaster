extern crate walkdir;
extern crate time;
extern crate rand;
extern crate fs_extra;
extern crate same_file;

use time::PreciseTime;

use std::fs;

pub fn search(dir: String) -> Vec<String> {
    let start = PreciseTime::now();
    
    println!("###### START SEARCH IN {} #######", &dir);
    let mut list : Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_name().to_str().unwrap().to_string().contains("jpg") | entry.file_name().to_str().unwrap().to_string().contains("JPG"){
                    list.push(entry.file_name().to_str().unwrap().to_string().to_uppercase());
                    }
                    else{ }
            }
        }
    }

    list.sort();

    println!("Directory: {}. Количество элементов: {}",dir, list.len());
    println!("Directory: {}. Вместимость: {}",dir, list.capacity());
    let end = PreciseTime::now();
    println!("###### END SEARCH. TIME: {} #######\n", start.to(end));
    return list;
}
