extern crate walkdir;
extern crate time;
extern crate rand;
extern crate fs_extra;
extern crate same_file;

use fs_extra::dir;
use fs_extra::dir::copy;
use fs_extra::copy_items;
use fs_extra::TransitProcess;
use time::PreciseTime;
use std::io;
use std::fs;
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, TryRecvError};
use std::path::Path;
use std::ffi::OsStr;

mod copypast;

////////////////////////// _____ CONSTANTS_____ //////////////////////////
// FIRSTFOLDER for test = C:\\Users\\druzhinin\\Desktop\\test1\\
const FIRSTFOLDER: &str = "N:\\TAMUZ8\\Pictures\\";

// SECONDFOLDER for test = C:\\Users\\druzhinin\\Desktop\\test2\\
// W:\\Tamuz8\\PICTURES\\
const SECONDFOLDER: &str = "C:\\Users\\druzhinin\\Desktop\\test1\\";

////////////////////////// _____ Entry point_____ //////////////////////////

fn main() {

    // let mut input_1 = String::new();
    // let mut input_2 = String::new();

    // match io::stdin().read_line(&mut input_1) {
    // Ok(n) => {
    //     println!("{} bytes read", n);
    //     println!("{}", input_1);
    // }
    // Err(error) => println!("error: {}", error),
    // }

    let start = PreciseTime::now();

    let first_vector: Vec<String> = copypast::search(FIRSTFOLDER.to_string());
    let second_vector: Vec<String> = copypast::search(SECONDFOLDER.to_string()); 

    compare(first_vector, second_vector);

    let end = PreciseTime::now();
    println!("TIME: {} seconds for whatever you did.",start.to(end));

    println!("Press eny key for Exit & Enter");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
            }
        Err(error) => println!("error: {}", error),
}
}

////////////////////////// _____ LOGIC_____ //////////////////////////
fn compare(first: Vec<String>, second: Vec<String>) {
    //let options = fs_extra::dir::CopyOptions {overwrite: false, skip_exist: true, buffer_size: 64000};
    let mut from_paths = Vec::new();

    //loop

    for x in 0..first.len(){
        let r = second.binary_search(&first[x]);
        let search_value = &first[x];
        
        match r {
            Ok(_search_value) => { 
                let mut copyfile = String::new();
                //copyfile.push_str(FIRSTFOLDER);
                copyfile.push_str(search_value);
                println!("НАЙДЕН: {} ", copyfile); },
            Err(_err) =>{
                println!("Файл:{}",search_value);
                let mut copyfile = String::new();
                //copyfile.push_str(FIRSTFOLDER);
                copyfile.push_str(search_value);
                from_paths.push(copyfile);
            },
        }
    }

    println!("Working...");

    copy_file_to_directory(from_paths);

    //let result = copy_items(&from_paths, SECONDFOLDER, &options);
    //let result = fs_extra::copy_items_with_progress(&from_paths, SECONDFOLDER, &options, handle);

    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     let handler = |process_info: TransitProcess| {
    //         println!("Total bytes: {}\nFile name: {}\n", &process_info.total_bytes, &process_info.file_name); 
    //         tx.send(process_info).unwrap();
    //         fs_extra::dir::TransitProcessResult::Skip
    //     };
    //     fs_extra::copy_items_with_progress(&from_paths, SECONDFOLDER, &options, handler).unwrap();
    // });

    // loop {
    //     match rx.try_recv() {
    //         Ok(process_info) => {
    //             println!("{} of {} bytes",
    //                      process_info.copied_bytes,
    //                      process_info.total_bytes);
    //         }
    //         Err(TryRecvError::Disconnected) => {
    //             println!("finished");
    //             break;
    //         }
    //         Err(TryRecvError::Empty) => {}
    //     }
    // }

    //println!("result = {}", &result.unwrap());

    // match result {
    // Ok(_res) => {println!("DONE!");},
    // Err(err) => {println!("ОШИБКА: {}. Описание: {}. Детали: {:?}",err, err.description(), err.cause());},
    // }
    }


fn copy_file_to_directory(from_paths: Vec<String>) {

    for file in from_paths {
        //let b = Path::new(&file).is_dir();

        // println!("{}",file);
        
         if (Path::new(&file).extension().unwrap_or(OsStr::new("other")) == "jpg") {

            //println!("file_src: {}", file);
            let mut file_first = String::new();
            file_first.push_str(FIRSTFOLDER);
            file_first.push_str(file.as_str());

            let b = Path::new(&file_first).is_dir();
            //println!("dir? {}", b);

            
            
            let mut file_second = String::new();
            file_second.push_str(SECONDFOLDER);
            file_second.push_str(file.as_str());
            
            if b!= true {
                println!("file_first: {}", file_first);
                println!("second_file: {}", file_second);
                let result = fs::copy(file_first, file_second);
                println!("num: {:?}", result);
                }
            }
        }
    }