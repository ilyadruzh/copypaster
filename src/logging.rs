extern crate chrono;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use self::chrono::prelude::*;

pub fn create_log(folder: String, mut log_data: Vec<String>) {

    let datetimenow: DateTime<Local> = Local::now();
    // let date_like_date: Date<Utc> = datetimenow.date();

    let date_like_string = datetimenow.to_string();

    let file_name: String = date_like_string + &".txt";
    let full_path_for_log = folder + &file_name.replace(":","-");

    let path = Path::new(&full_path_for_log);
    let display = path.display();

    println!("Пишем лог...");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };
    
    for x in 0..log_data.len(){
        match writeln!(file, "{}\r", log_data[x]) {
            Err(why) => {
                panic!("couldn't write to {}: {}", display, why.description())
                },
            Ok(_) => (),//println!("successfully wrote to {}", display),
        }
    }

    // for x in 0..log_data.len(){
    //    log_data[x].push('\n');

    //match file.write_all(log_data[x].as_bytes()) {
    //    Err(why) => {
    //        panic!("couldn't write to {}: {}", display, why.description())
    //    },
    //    Ok(_) => (),//println!("successfully wrote to {}", display),
    //}
    //}
}
