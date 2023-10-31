use std::fs::File;
use std::io::{self, BufRead, Write};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn read_file_lines(file_path: String) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn save(file_path: String, lines: &Vec<String>){
    let mut file = File::create(file_path).unwrap();

    for line in lines {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}

fn insert(mut lines: Vec<String>, line_index: usize) {
    let mut line_input = String::new();
    println!("what do you want to write?");
    io::stdin().read_line(&mut line_input).expect("failed to read content of line to insert");

    let len = lines.len();

    if line_index > len{
        for _n in 1 .. line_index - len{
            lines.push("".to_string());
        }
        lines.push(line_input);
    }else{
        lines.insert(line_index-1, line_input);
    }
    
    //inserted values have \r\n once theyre inserted, its annoying make it stop
}

fn append(mut lines: Vec<String>) {
    let mut line_input = String::new();
    println!("what do you want to write?");
    io::stdin().read_line(&mut line_input).expect("failed to read content of line to insert");  

    lines.push(line_input);
}

fn delete(mut lines: Vec<String>, line_index: usize) {
    let len = lines.len();
    
    if line_index < len{
        lines.remove(line_index-1);
    }else{
        eprint!("index is out of range of the file");
    }
}

fn copy(lines: Vec<String>, line_index: usize) {
    let len = lines.len();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    if line_index < len{
        ctx.set_contents(lines[line_index].to_owned()).unwrap();
    }else{
        eprint!("index is out of range of the file");
    }
}    

fn move_line(mut lines: Vec<String>, line_index1: usize, line_index2: usize) {
    let contents = lines[line_index1].clone();

    let len = lines.len();

    if line_index1 < len {
        if line_index2 < len {
            lines.insert(line_index2-1, contents);
        }else{
            for _n in 1 .. line_index2 - len{
                lines.push("".to_string());
            }
            lines.push(contents);
        }
        lines.remove(line_index1-1);
    }else{
        eprint!("index is out of range of the file");
    }
}

fn display(lines: Vec<String>, line_index1: usize, line_index2: usize) {
    println!("output:");
    for n in line_index1-1 .. line_index2 {
        println!("{}",lines[n]);
    }
}

fn help() {
    println!("p n1 n2: display lines n1 to n2");
    println!("i n: insert message at line n1");
    println!("a: append at end of file");
    println!("m n1 n2: move line n1 to n2");
    println!("c n: copy line n");
    println!("f str: find string str in file");
    println!("s: save changes to file");
    println!("h: help");
    println!("q: quit\n");
}

fn main() {
    let mut file_path = String::new(); 

    println!("(type h in prompt for help)\n\nwhat file would you like to open?");
    io::stdin().read_line(&mut file_path).expect("failed to read file name input");

    file_path = "e.txt".to_string();

    let lines = match read_file_lines(file_path.clone()) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("error: {}", e);
            return;
        }
    };
    
    let line_count = lines.len();

    println!("\nlines: {line_count}");

    loop{
        let mut input = String::new();
        let mut index: usize = 0; 
        let mut index2: usize = 0;
        let file_path_clone = file_path.clone();

        print!("? ");
        io::stdin().read_line(&mut input).expect("failed to read input");

        let values: Vec<&str> = input.split_whitespace().collect();

        println!("{:?}", values);

        if values.len() == 2{
            index = values[1].trim().parse().unwrap();
        }else if values.len() == 3{
            index = values[1].trim().parse().unwrap();
            index2 = values[2].trim().parse().unwrap();
        }else{
            println!("argument structure incorrect");
        }

        match values[0].trim() {
            "p" => {
                display(lines.clone(), index, index2);
            },
            "a" => {
                append(lines.clone());
            },
            "i" => {
                insert(lines.clone(), index);
            },
            "d" => {
                delete(lines.clone(), index);
            },
            "c" => {
                copy(lines.clone(), index);
            },
            "s" => {
                save(file_path_clone, &lines);
            },
            "m" => {
                move_line(lines.clone(), index, index2);
            },
            "h" => {
                help();
            },
            "q" => {
                return;
            }, 
            &_ => {
                eprint!("incorrect input");
            }
        }

    }
}
