use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;


enum FileState {
    Open,
    Closed
}



impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED")
        }
    }
}

impl Debug for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => {
                f.debug_struct("FileState")
                 .field("Open", &FileState::Open)
                 .finish()
            },
            FileState::Closed => {
                f.debug_struct("FileState")
                 .field("Closed", &FileState::Closed)
                 .finish()
            }
        }
    }
}



struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}



impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
         .field("name", &self.name)
         .field("data", &self.data)
         .field("state", &self.state)
         .finish()
    }
}

// impl PartialEq<FileState> for &File {
//     fn eq(&self, other: &FileState) -> bool {
//         self.state == *other
//     }
// }

// impl PartialEq<&File> for FileState {
//     fn eq(&self, other: &&File) -> bool {
//         *self == other.state
//     }
// }

// impl PartialEq<FileState> for FileState {
//     fn eq(&self, other: &FileState) -> bool {
//         let mut a = false;
//         let mut b = false;
//         match self {
//             FileState::Open => {a = true;},
//             _ => {}
//         }
//         match other {
//             FileState::Open => {b = true;},
//             _ => {}
//         }
//         if a && b {
//             true
//         } else {
//             false
//         }
//     }
// }

impl PartialEq for FileState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Open, Self::Open) => true,
            (Self::Closed, Self::Closed) => true,
            _ => false
        }
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be opened for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn close(mut f:File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f1 = File::new("test.txt");
    assert!(1 == 1);
    assert!(f1.state == FileState::Closed);
}