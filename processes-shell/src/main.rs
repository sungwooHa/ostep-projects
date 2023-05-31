use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const WISH: &str = "wish> ";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();

    if args_len == 1 {
        action_in_mode()?;
    } else if args_len > 1 {
        action_in_file(args[1..].to_vec())?;
    }

    Ok(())
}

fn action_in_mode() -> std::io::Result<()> {
    Ok(())
}

fn action_in_file(args: Vec<String>) -> std::io::Result<()> {
    for file_name in args {
        println!("file name : {}", file_name.as_str());

        let file = File::open(file_name.as_str())?;
        let buf_reader = BufReader::new(file);

        for line in buf_reader.lines() {
            match line {
                Ok(line) => {}
                Err(_) => {}
            }
        }
    }

    Ok(())
}

fn action_in_line(line : String) -> std::io::Result<()> {

    Ok(())
}