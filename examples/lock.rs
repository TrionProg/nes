//This is an example, how to work with Mutex or RwLock

#[macro_use]
extern crate nes;
use nes::{ErrorInfo,ErrorInfoTrait};

use std::sync::{Mutex,Arc,RwLock};
use std::thread;


define_error! (Error,
    Poisoned() => "poisoned",
    PoisonedWithArgs(who:String) => "Mutex has been poisoned by thread \"{}\""
);

fn thread_function(common:Arc<Mutex<u32>>) -> result![Error] {
    let guard=mutex_lock!(common);

    if *guard<10 {
        panic!("Panic");
    }

    ok!()
}

fn main_function() -> result![Error] {
    let common=Arc::new(Mutex::new(1u32));
    let thread_common=common.clone();

    let join_handle=thread::spawn(||{
        thread_function(thread_common).unwrap();
    });

    thread::sleep(std::time::Duration::new(1,0));

    //Is poisoned
    //let value_guard=mutex_lock!(common);
    //let value_guard=mutex_lock!(common,Error);
    //let value_guard=mutex_lock!(common,Error::Poisoned);
    let value_guard=mutex_lock!(common,Error::PoisonedWithArgs,"child".to_string());

    println!("Value is {}",*value_guard);

    join_handle.join();

    ok!()
}

fn main() {
    match main_function() {
        Ok(_) => {},
        Err(e) => println!("The problem has occurred, we must solve it\n{}",e),
    }
}
