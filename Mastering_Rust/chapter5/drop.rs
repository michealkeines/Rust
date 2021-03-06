struct Character {
    name: String
}

impl Drop for Character {
    fn drop(&mut self) {
        println!("{} went way", self.name);
    }
}

fn main() {
    let steve = Character {
        name: "Steve".into()
    };

    let john = Character {
        name: "John".into()
    };
}