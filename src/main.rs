use core::panic;
use std::collections::btree_set::Intersection;
use std::fs::File;
use std::io;
use std::fs;
use std::path::PathBuf;
use std::thread::panicking;

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
        print!("\x1B[2J\x1B[H");

        self.make_one_letter(self.count);
        self.count += 1;            

        println!("TEMP");
        for line in self.temp.iter() {
            println!("{}", line);            
        }        

    }

    fn make_one_letter(&mut self, nr: u32) {

        //dev config
        let replacment_letter: char = 'z';



        let mut seen_letters: Vec<char> = Vec::new();
        let mut count = 0;
        // reads the file contents and chooses nr letters that has not been seen.
        'outer: for line in self.mem.iter() {
            for c in line.chars() {
                if count == nr {
                    break 'outer;
                }
                if !seen_letters.contains(&c) {
                    seen_letters.push(c);
                    count += 1;
                    println!("{:?}", seen_letters)
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
    }
}

fn display_directorys_txt_files() -> Vec<PathBuf> {     

    let mut vec: Vec<PathBuf> = Vec::new();
    let mut i = 0;
    println!("Found txtFiles:");
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

        i +=1;
    }
    vec

}

fn main() {
    let mut file_holder = FileHolder {
        mem: Vec::new(),
        count: 1,
        ..Default::default()
    };    
    
    let vec = display_directorys_txt_files();

    //let the user choose a file out of the found txt files.
    let mut userChoise: String = String::new();    
    println!("Choose correct index for your txt file: ");
    io::stdin()
    .read_line(&mut userChoise)
    .expect("Failed to read user input");

    let index: i32 = userChoise
    .trim()
    .parse()
    .expect("Failed to convert user input to number");

    while (vec.len() as i32) >= index {
        panic!("user enterd index exceeds the maximum index");
    }

    let path = vec[4];


    /*
    //read file contents into File struct
    for line in read_to_string(path).unwrap().lines() {
        //put each line into a vector in FileHolder
        file_holder.load_into_memory(line);
        
    }

    let mut input = String::new();
    loop {
        file_holder.display();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");        
    }
     */
}
