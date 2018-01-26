extern crate reqwest;

use std::env;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: ip:port");
        return;
    }
    let ref address = args[1];
    println!("The server address is: {}", address);
    let size: usize = 20;
    let mut children =  Vec::with_capacity(size);

    for _ in 0..size {
        let address = address.clone();
        let child = thread::spawn(move || {
            let mut resp = reqwest::get(&address).unwrap();
            let res = resp.status().is_success();
            if res {
                let body = resp.text();
                println!("body = {:?}", body);
            }
        });
        children.push(child);
    }

//    for (i, child) in children.iter().enumerate() {
//        let res = child.join().unwrap();
//        println!("Successfully join child {}", i);
//    }
//

    children.into_iter()
            .map(|child| {child.join().unwrap()}).collect::<Vec<_>>();

//    for child in children.into_iter() {
//        let res = child.join().unwrap();
//    }


    return;
}
