use std::env;
use std::collections::HashMap;
use percent_encoding::percent_decode_str;

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }
    
    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Tweakable capacity
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| if u == 0 { None } else { Some(Rc::clone(&self.buf)) })
                .transpose()
        }
    }
}

fn paramizer(url:&str) -> Option<Vec<&str>> {
    let query_str = url.split("?").nth(1);
    if let Some(qs) = query_str {
       qs.split("&").map(|s| s.split("=").next()).collect()
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("need a filename");
    }
    let filename = &args[1];
    let mut count: usize = 0;
    let mut param_map: HashMap<String, usize> = HashMap::new();

    for line in my_reader::BufReader::open(filename)? {
        let url = percent_decode_str(&line?).decode_utf8_lossy().to_string();
        for param in paramizer(&url).iter().flatten() {
            if !param_map.contains_key(*param) {
                param_map.insert(param.to_string(),count);
                count+=1;
                println!("{}",param.trim());
            }
        }
    }
    println!("----------------------------------------------------------");
    println!("Parameters found: {}", count);

    Ok(())
}
