extern crate data_structure;
use data_structure::queue::Queue;

pub fn run() {
    let mut a:Queue<i32> = Queue::new();
    a.summary();
    for i in 0..9 as i32 {
        a.en_queue(i);
        a.summary();
    }

    let d = a.de_queue();
    a.summary();
    println!("Dequeue: {}", d);
}
