use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};

fn process_reader<R: BufRead>(reader: R, needle: &str, ctx_lines: usize) {
    let mut haystack: String = String::new();
        for line in reader.lines().map(|l| l.unwrap()) {
          //  println!("line {:?}", line);
            let mut v = line;
            v.push_str("\n");
            haystack.push_str(&v);
        }
    
        let mut tags: Vec<usize> = vec![];
        let mut ctx: Vec<Vec<(usize, String)>> = vec![];
        
       // println!("updated");

        for (i, line) in haystack.lines().enumerate() {
        //     if line.contains(needle) {
        //         tags.push(i);
        //         let v = Vec::with_capacity(2 * ctx_lines + 1);
        //         ctx.push(v);
        //     }
        let re = Regex::new(needle).unwrap();
            let contains_substring = re.find(line);

            match contains_substring {
                Some(_) => {
                    tags.push(i);
                    let v = Vec::with_capacity(2 * ctx_lines + 1);
                    ctx.push(v);
                },
                None => ()
            }
        }
        if tags.is_empty() { return; }

        for (i, line) in haystack.lines().enumerate() {
            for (j, tag) in tags.iter().enumerate() {
                let lower_bound = tag.saturating_sub(ctx_lines);
                let upper_bound = tag + ctx_lines;
                if (i >= lower_bound) && (i <= upper_bound) {
                    let line_as_string = String::from(line);
                    let local_ctx = (i, line_as_string);
                    ctx[j].push(local_ctx);
                }
            }
        }

        for local_ctx in ctx.iter() {
            for &(i, ref line) in local_ctx.iter() {
                let line_um = i + 1;
                println!("{}: {}", line_um, line);
            }
        }
}

fn main() {
    // let  search_term = "picture";
    // let quote = "\
    // Every face, every shop, bedroom window, public-house, and
    // dark square is a picture feverishly turned--in search of what?
    // It is the same with books. what do we seek through millions of pages?";

    // for (i, line) in quote.lines().enumerate() {
    //     if line.contains(search_term) {
    //         let line_num = i + 1;
    //         println!("{}: {}", line_num, line);
    //     }
    // }
    let args = App::new("grep-lite")
                    .version("0.1")
                    .about("searches for patters")
                    .args(&[
                        Arg::with_name("pattern")
                        .help("The pattern to search for")
                        .takes_value(true)
                        .required(true),
                        Arg::with_name("filename")
                        .help("Enter the input filename to read from")
                        .takes_value(true)
                        .required(true),
                        Arg::with_name("getlines")
                        .help("Enter the number of lines to print above and below the found line")
                        .takes_value(true)
                        .required(true)
                        ]
                    )
                    .get_matches();


    let ctx_lines: usize = args.value_of("getlines").unwrap().parse().unwrap();
    let needle = args.value_of("pattern").unwrap();

    let fname = args.value_of("filename").unwrap();

    if fname == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_reader(reader, needle, ctx_lines)
    } else {
        let f = File::open(fname).unwrap();
        let reader = BufReader::new(f);
        process_reader(reader, needle, ctx_lines)
    }
    
    
}