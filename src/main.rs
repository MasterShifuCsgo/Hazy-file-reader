use std::fs::{self, read_to_string, File};
use std::hash::Hash;
use std::{io, vec};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
#[derive(Default)]
struct FileHolder {
    mem: Vec<String>,
    temp: Vec<String>,
    count: u32,
}

impl FileHolder {
    fn load_into_memory(&mut self, s: &str) {
        self.mem.push(s.to_string());
    }

    fn display(&mut self) {
        self.make_one_letter(self.count);
        self.count += 1;
        
        for line in self.temp.iter() {
            println!("{}", line);
        }
        self.temp.clear();
    }

    fn make_one_letter(&mut self, nr: u32) {
        


        //dev config        
        let replacment_letter: char = 'z';
        let mut seen_letters = HashSet::new();
        let mut count = 0;

        // reads the file contents and chooses nr letters that has not been seen.
        'outer: for line in self.mem.iter() {
            for c in line.chars() {
                if count == nr {
                    break 'outer;
                }
                if seen_letters.insert(c) {                    
                    count += 1;
                }
            }
        }

        // read each line and make it the letter if it does not correspond to the letter already taken.
        for line in self.mem.iter() {
            let mut temp_line: String = String::new();
            for c in line.chars() {
                if seen_letters.contains(&c) && c != replacment_letter {
                    temp_line.push(c);
                } else {
                    temp_line.push(replacment_letter);
                }
            }
            self.temp.push(temp_line);
        }

        print!("Amount of uniqe letters: {}", seen_letters.len());

    }
}

fn display_directorys_txt_files() -> Vec<PathBuf> {
    let mut vec: Vec<PathBuf> = Vec::new();
    let mut i = 0;
    println!("Found txt files:");
    for entry in fs::read_dir("./src").expect("File not found") {
        let dir: PathBuf = entry.expect("Entry Failiure; ").path();

        //check if file is txt
        let mut is_txt: bool = false;
        if dir.is_file() {
            if let Some(ext) = dir.extension() {
                is_txt = ext == "txt";
            }
        }

        if is_txt {
            println!("{} {}", i, dir.display()); //file index, file directory
            vec.push(dir);
        }

        i += 1;
    }
    vec
}

//let the user choose a file out of the found txt files.
fn select_file() -> PathBuf {
    let mut vec = display_directorys_txt_files();
    let mut user_choise: String = String::new();
    println!("Choose correct index for your txt file: ");
    io::stdin()
        .read_line(&mut user_choise)
        .expect("Failed to read user input");

    let index: usize = user_choise
        .trim()
        .parse()
        .expect("Failed to convert user input to number");

    if index >= vec.len() {
        panic!("User entered index exceeds the maximum index");
    }    

    vec.into_iter()
    .nth(index)
    .expect("Index out of bounds")
}

fn run_loop(file_holder: &mut FileHolder){
    let mut input: String = String::new();
    loop {
        file_holder.display();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "exit" {
            break;
        }
    }
    input.clear();
}

fn load_content_to_memory(path: PathBuf, file_holder: &mut FileHolder) {
//read file contents into File struct
    for line in read_to_string(path).unwrap().lines() {
        //put each line into a vector in FileHolder
        file_holder.load_into_memory(line);
    }
}

fn main() {
    let mut file_holder: FileHolder = FileHolder {
        mem: Vec::new(),
        count: 1,
        ..Default::default()
    };        
    let path: PathBuf = select_file();
    load_content_to_memory(path, &mut file_holder);
    run_loop(&mut file_holder);
    
}
