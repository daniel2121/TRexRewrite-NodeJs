use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem};
use tesla::Event;

#[derive(Clone)]
struct SingletonQueues { inner: Arc<Mutex<HashMap<String, Vec<Arc<Event>>>>>}

fn singletonqueues() -> SingletonQueues {
    static mut SINGLETON: *const SingletonQueues = 0 as *const SingletonQueues;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            let singletonqueues = SingletonQueues {
                inner: Arc::new(Mutex::new(HashMap::new()))
            };
            SINGLETON = mem::transmute(Box::new(singletonqueues));
        });
        (*SINGLETON).clone()
    }
}

pub fn insert_queue(connid: String, event: Arc<Event>){
    let s = singletonqueues();
    let mut conn_queues = s.inner.lock().unwrap();

    let queue = conn_queues.entry(connid).or_insert(vec![]);

    (*queue).insert(0,event);
}

pub fn pop_queue(connid: String) -> Option<Arc<Event>> {
    let s = singletonqueues();
    let mut conn_queues = s.inner.lock().unwrap();

    if conn_queues.contains_key(&connid) == true {
        let queue = conn_queues.entry(connid).or_insert(vec![]);
        (*queue).pop()
    } else {
        None
    }
}

pub fn init_queue(connid: String) -> Option<Arc<Event>> {
    let s = singletonqueues();
    let mut conn_queues = s.inner.lock().unwrap();
    let queue = conn_queues.entry(connid).or_insert(vec![]);
    (*queue).pop()
}

pub fn remove_queue(connid: String){
    let s = singletonqueues();
    let mut conn_queues = s.inner.lock().unwrap();
    conn_queues.remove(&connid);
}

pub fn print_queue_status(){
    let s = singletonqueues();
    let conn_queues = s.inner.lock().unwrap();
    println!("Queue Status:");
    println!("Number of queues/conn_ids: {}", conn_queues.len());

    println!("Connection IDs in use:");
    for conn_id in conn_queues.keys() {
        println!("  - {}" , conn_id);

        for i in 0..conn_queues[conn_id].len() {
            println!("     * {:?}", conn_queues[conn_id][i]);
        }
    }
}

pub fn write_status() -> Result<(), io::Error>{
    let s = singletonqueues();
    let conn_queues = s.inner.lock().unwrap();
    let mut buffer = try!(File::create("status.txt"));

    try!(buffer.write_all(format!("Queue Status:\n").as_bytes()));

    try!(buffer.write_all(format!("Number of queues/conn_ids: {}\n", conn_queues.len()).as_bytes()));

    try!(buffer.write_all(format!("Connection IDs in use:\n").as_bytes()));

    for conn_id in conn_queues.keys() {
        try!(buffer.write_all(format!("  - {}\n" , conn_id).as_bytes()));

        for i in 0..conn_queues[conn_id].len() {
            try!(buffer.write_all(format!("     * {:?}\n", conn_queues[conn_id][i]).as_bytes()));
        }
    }

    Ok(())
}
