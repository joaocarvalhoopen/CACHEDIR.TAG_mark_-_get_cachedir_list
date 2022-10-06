// Name:    get_cachedir_list
// Author:  Joao Nuno Carvalho
// Date:    2022.10.06
//
// Description: This is a little program that lists on a file one line for each
//              directory entry the CACHEDIR.TAG mark on the files that are of
//              the type cache.
//              For information about cache directory tags see
//              https://bford.info/cachedir/
//              The search is made for the sub-directories of the given
//              directory.
//              The target for the time being is Linux.
//
// Usage example:
//      get_cachedir_list  <search dir> <dir to place the result file>
//
//   ex:
//      It writes by default on /dev/shm RAM Disk on Linux.
//      get_cachedir_list  /home/<user>
//
//   ex:
//      You can also specify the directory to write to.
//      get_cachedir_list  /home/<user> /dev/shm/bla 
//
// License: MIT Open Source License.
//


use std::env;
use walkdir::WalkDir;
use std::fs;

const HEADER_STR:   &'static str = "Signature: 8a477f597d28d172789f06886806bc55";
const CACHEDIR_TAG: &'static str = "CACHEDIR.TAG";
const CACHEDIR_TAG_REMOVE: &'static str = "CACHEDIR.TAG";

fn main() {
    // Parses path argument.
    let args: Vec<String> = env::args().collect();

    // debug
    println!("args.len()= {}", args.len());

    if args.len() != 2 && args.len() != 3 {
        print_exit();
        std::process::exit(-1);
    }

    let initial_dir_path = & args[1];
    let mut res_write_path = "/dev/shm".to_string();

    if args.len() == 3 {
        res_write_path = args[2].clone();
        // Removes the last slash.
        if res_write_path.ends_with("/") {
            res_write_path.remove(res_write_path.len() - 1);
        }  
    }

    let mut exclusion_path_list = String::with_capacity(10_000);

    let walker = WalkDir::new(initial_dir_path).into_iter();
    for entry in walker {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap() == CACHEDIR_TAG {
            match fs::read_to_string(entry.path()) {
                // Converts the file into a String, and deserialize into a structure. 
                Ok(text_string) => {
                    if text_string.starts_with(HEADER_STR) {
                        exclusion_path_list.push_str(
                            & entry.path().to_str().unwrap()
                            .replace(CACHEDIR_TAG_REMOVE, ""));
                        exclusion_path_list.push_str("\n" );
                    }
                },
    
                Err(err_str) => {
                   println!("Error: While reading config file ... {}", err_str);
                   return;
                }
             }
        }

        // println!("{}", entry.path().display());
        print!(".");
    }

    let res_path = res_write_path +  "/exclusion_dirs.txt";
    let res = fs::write(& res_path, exclusion_path_list);
    if res.is_err() {
        println!("Error while writing file\n{}.", res_path);
    } else {
        println!("\nFile written to\n{}", & res_path);
        println!("...end.");
    }
}

fn print_exit() {
    println!("Error:\n  Please pass the starting directory path.");
    println!("   get_cachedir_list  <search dir> <dir to place the result file>");
    println!("\n ex:");
    println!("   get_cachedir_list  /home/<user>  ");
    println!("\n ex:");
    println!("   get_cachedir_list  /home/<user> /dev/shm ");
}
