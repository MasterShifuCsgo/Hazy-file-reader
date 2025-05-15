use std::fs::read_to_string;
use std::io;
use std::path::Path;

#[derive(Default)]
struct FileHolder {
    mem: Vec<String>,
    temp: Vec<String>,
    letter: char,
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
                if seen_letters.contains(&c) {
                    temp_line.push(c);
                } else {
                    temp_line.push(self.letter);
                }                
            }
            self.temp.push(temp_line);
        }
    }
}

fn main() {
    let mut file_holder = FileHolder {
        mem: Vec::new(),
        letter: 'x',
        count: 1,
        ..Default::default()
    };

    let path = Path::new("./hello_world.txt"); // takes the current path the .exe file is in

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
}
