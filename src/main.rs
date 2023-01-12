use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader, Write, self};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let matches = App::new("compare_files")
        .version("1.0")
        .author("you")
        .about("compare number of lines and ratio of two files")
        .arg(Arg::with_name("file1")
             .help("first file to compare")
             .required(true)
             .index(1))
        .arg(Arg::with_name("file2")
             .help("second file to compare")
             .required(true)
             .index(2))
        .get_matches();

    let file1 = matches.value_of("file1").unwrap();
    let file2 = matches.value_of("file2").unwrap();

    loop {
        let file1 = File::open(file1).expect("Couldn't open file1");
        let file2 = File::open(file2).expect("Couldn't open file2");

        let file1 = BufReader::new(file1);
        let file2 = BufReader::new(file2);

        let lines1 = file1.lines().count();
        let lines2 = file2.lines().count();
        let ratio = (lines1 as f64 / lines2 as f64) * 100.0;
        print!("\r{:.2}% - {:?} of {:?}", 
                ratio, lines1, lines2);
        io::stdout().flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
