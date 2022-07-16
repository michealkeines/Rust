use futures::executor::block_on;
use futures::Future;

fn print_type<T> (_: T) {
    println!("{}", std::any::type_name::<T>());
}

async fn hello_world() {
    println!("Hello, World!");
}

use std::time::Duration;
use std::thread;

async fn learn_song() -> String {
    println!("learning song");
    thread::sleep(Duration::from_secs(1));
    println!("Song learned");
    "testsong".to_string()
}

async fn sing_song(song: String) {
    println!("new song request");
    thread::sleep(Duration::from_secs(3));
    println!("song over");
}

async fn dance() {
    println!("dancing");
    thread::sleep(Duration::from_secs(1));
    println!("done dancing");
}


fn main() {
//     println!("Started");
//     let future = hello_world();
//    // print_type(&future);
//     println!("yet to be printed");
//     block_on(future);

    let learn_song_f = learn_song();
    let dance_f = dance();
    
    let song = block_on(learn_song_f);
    let sing_song_f = sing_song(song);

    block_on(sing_song_f);
    block_on(dance_f);
}