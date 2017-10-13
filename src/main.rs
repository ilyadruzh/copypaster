extern crate walkdir;
extern crate time;
extern crate rand;
extern crate fs_extra;
extern crate log;
extern crate env_logger;
extern crate winapi;
extern crate user32;
extern crate kernel32;

use fs_extra::dir;
use fs_extra::TransitProcess;
use time::PreciseTime;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, TryRecvError};
use std::io;

mod copypast;
mod logging;


////////////////////////// _____ CONSTANTS_____ //////////////////////////
const FIRSTFOLDER: &str = r"\\Serv02\base\TAMUZ8\Pictures\";

// FOR TEST = C:\Users\druzhinin\Desktop\test2\
// PRODUCTION = \\Serv17\base\Tamuz8\PICTURES\
const SECONDFOLDER: &str = r"\\Serv17\base\Tamuz8\PICTURES\";
const FOLDERFORLOG: &str = r"N:\TamuzXpa\8_Procedures\CopyPicturesFromMoscowToPerm\";

////////////////////////// _____ Entry point_____ //////////////////////////
fn main() {

    let start = PreciseTime::now();
    let first_vector: Vec<String> = copypast::search(FIRSTFOLDER.to_string());
    let second_vector: Vec<String> = copypast::search(SECONDFOLDER.to_string()); 
    
    compare(first_vector, second_vector);

    let end = PreciseTime::now();

    println!("ОБЩЕЕ ВРЕМЯ РАБОТЫ ПРОГРАММЫ: {} секунд.", start.to(end));

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
    let options = dir::CopyOptions::new();
    let mut from_paths = Vec::new();
    let mut vec_for_log: Vec<String> = Vec::new();

    for x in 0..first.len(){

        let r = second.binary_search(&first[x]);
        let search_value = &first[x];
        let log_value = first[x].clone();
        
        match r {            
            Ok(_search_value) => { 
                //println!("_____НАШЕЛ_____ ::: {}",&search_value);
                let mut existfile = String::new();
                existfile.push_str(FIRSTFOLDER);
                existfile.push_str(search_value);
                },
            Err(_err) =>{    
                println!("##### НЕ НАШЕЛ ##### ::: {}", search_value);
                let mut copyfile = String::new();
                copyfile.push_str(FIRSTFOLDER);
                copyfile.push_str(search_value);
                vec_for_log.push(log_value);
                from_paths.push(copyfile);
            },
        }
    }

    println!("Количество новых файлов: {}", &from_paths.len());

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let handler = |process_info: TransitProcess| {
            match tx.send(process_info) {
                Ok(result) => { 
                    result
                }
                Err(err) => {println!("Ошибка: {:?}", err)}
            };
            thread::sleep(Duration::new(0,500));
            fs_extra::dir::TransitProcessResult::Skip
        };
        fs_extra::copy_items_with_progress(&from_paths, SECONDFOLDER, &options, handler).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                println!("скопировано {} из {} байтов\r",
                         process_info.copied_bytes,
                         process_info.total_bytes);
            }
            Err(TryRecvError::Disconnected) => {
                println!("finished");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }

    if !vec_for_log.is_empty() { logging::create_log(FOLDERFORLOG.to_string(), vec_for_log); }
}