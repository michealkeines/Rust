use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new()
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(&self, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10) {
        let err = String::from("Permission denied");
        return Err(err);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100) {
        let err = String::from("Interrupted by signal");
        return Err(err);
    }
    Ok(f)
}


fn main() {
    let f3_data: Vec<u8> = vec![
        114, 117, 115, 116, 33
    ];
    let mut f3 = File::new_with_data("f3.txt", &f3_data);

    let mut buffer: Vec<u8> = vec![];

    f3 = open(f3).unwrap();
    let f3_length = f3.read( &mut buffer);
    f3 = close(f3).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, &f3_length);
    println!("{}", text);
}
