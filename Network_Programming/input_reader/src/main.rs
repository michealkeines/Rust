use futures::executor::block_on;
use futures::Future;
use rand::Rng;

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
 //   thread::sleep(Duration::from_secs(1));
    for i in 0..10 {
        println!("{} in learn_song",i);
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(100,500)));
    }
    println!("Song learned");
    "testsong".to_string()
}

async fn sing_song(song: String) {
    println!("new song request");
  //  thread::sleep(Duration::from_secs(3));
    for i in 0..25 {
        println!("{} in sing_song",i);
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(700,900)));
    }
    println!("song over");
}

async fn dance() {
    println!("dancing");
  //  thread::sleep(Duration::from_secs(1));
    for i in 0..10 {
        println!("{} in dance",i);
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(100,300)));
    }
    println!("done dancing");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn run_all_futures() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn main() {
//     println!("Started");
//     let future = hello_world();
//    // print_type(&future);
//     println!("yet to be printed");
//     block_on(future);

    // let learn_song_f = learn_song();
    // let dance_f = dance();
    
    // let song = block_on(learn_song_f);
    // let sing_song_f = sing_song(song);

    // block_on(sing_song_f);
    // block_on(dance_f);

    block_on(run_all_futures());
}