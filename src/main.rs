use std::env::args;

mod my_image;

fn main() {
    let mut arguments: Vec<String> = args().collect();
    arguments.remove(0);

    let infile: String;
    let outfile: String;
    match arguments.len() {
        1 => {
            infile = arguments[0].clone();
            outfile = infile.clone();
        }
        2 => {
            infile = arguments[0].clone();
            outfile = arguments[1].clone();
        }
        _ =>  {
            println!("Invalid number of arguments \nUsage: qoi.exe <input> [output]");
            return;
        }
    }
    println!("{infile}, {outfile}");
}
