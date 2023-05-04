use std::{env::args};
use my_image::Image;

mod my_image;

fn main() {
    let mut arguments: Vec<String> = args().collect();
    arguments.remove(0);

    let infile: String;
    let mut outfile: String;
    match arguments.len() {
        1 => {
            infile = arguments[0].clone();
            outfile = infile.clone()/* .split(".").next().unwrap().to_owned(); */
        }
        2 => {
            infile = arguments[0].clone();
            outfile = arguments[1].clone();
        }
        _ => {
            println!("Invalid number of arguments \nUsage: qoi.exe <input> [output]");
            return;
        }
    }

    let infiletype = infile.split('.').last().unwrap().to_owned();

    let outfiletype: String;
    if outfile != infile {
        let outfile_parts = outfile.split('.');
        if outfile_parts.clone().count() == 1 {
            if infiletype != "qoi" {
                outfiletype = String::from("qoi");
            } else {
                outfiletype = String::from("png");
            }
        } else {
            outfiletype = outfile.split('.').last().unwrap().to_owned();
        }
    } else {
        if infiletype != "qoi" {
            outfiletype = String::from("qoi");
        } else {
            outfiletype = String::from("png");
        }
        
        outfile = outfile.split('.').next().unwrap().to_owned();
        outfile.push_str(format!(".{outfiletype}").as_str());
    }

    println!("{infile}, {outfile}, {infiletype}, {outfiletype}");

    let img: Image = match infiletype.as_str() {
        "qoi" => Image::read_qoi(&infile).unwrap(),
        "png" => Image::read_png(&infile).unwrap(),
        _ => { println!("Unsupported input file type!"); return; }
    };
}
