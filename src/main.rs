use std::{env, fs::File, io::{self, BufRead, BufReader}};
use colored::{control::set_override, *};

fn highlight(line: &str, query: &str) -> String {
    let lower_line = line.to_lowercase();
    let lower_query = query.to_lowercase();

    let mut result = String::new();
    let mut i = 0;

    while i < line.len() {
        if lower_line[i..].starts_with(&lower_query) {
            let original = &line[i..i + query.len()];

            result.push_str(&original.red().to_string());

            i += query.len();
        } else {
            result.push(line[i..].chars().next().unwrap());
            i += 1;
        }
    }

    result
}

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    set_override(true);
    if args.len() < 2 {
        eprintln!("Usage: {} <query> [filename]",args[0]);
        eprintln!("IF no filename is provided, reads from stdin");
        return Ok(());
    }

    let query = &args[1];
    if let Some(file_name) = args.get(2) {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
             if line.to_lowercase().contains(query){
                let highlighted = highlight(&line, query);
                println!("{}", highlighted);
             }
        }
    } else {
        let stdin = io::stdin();
        for line in stdin.lines(){
            let line = line?;
            if line.to_lowercase().contains(query){
                let highlighted = highlight(&line, query);
                println!("{}", highlighted);
            }
        }
    }
    Ok(())
}