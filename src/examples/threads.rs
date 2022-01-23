use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn compute_independent_tasks_with_basic_threading() {
    let v = Arc::new(vec![1, 2, 3, 4, 5]);
    let sum_v = Arc::clone(&v); // reference cloning, much cheaper than `v.clone()`
    let product_v = Arc::clone(&v);

    // get sum of the 'v'
    let sum_handle = thread::spawn(move || {
        let mut sum = 0;
        for i in sum_v.iter() {
            println!("iteration in sum thread");
            sum += i;
        }
        sum
    });
    // can't use `v` here, it has been moved to closure

    // sum_handle.join().unwrap(); // main thread will wait for 'sum' thread
    // to finish, then will spawn 'product' thread

    // get product of the 'v'
    let product_handle = thread::spawn(move || {
        let mut product = 1;
        for i in product_v.iter() {
            println!("iteration in product thread");
            product *= i;
        }
        product
    });

    let sum = sum_handle.join().unwrap();
    let product = product_handle.join().unwrap();

    println!("Sum: {}, product: {}", sum, product);
}

use std::sync::mpsc;
//  “Do not communicate by sharing memory;
//   instead, share memory by communicating.” - from Go documentation
fn run_message_passing_between_threads_example() {
    // mpsc = multiple producer, single consumer
    // tx = transmitter, rx = reciever
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
            // tx.send(val).unwrap(); // `val` was moved by sending, won't compile
        }
    });
    // main thread will block and iterate rx
    // until the channel is not closed
    for received in rx {
        println!("Got: {}", received);
    }
}

use std::sync::Mutex;
// mutex = 1 thread at a time
fn run_mutex_example() {
    // Arc::new(0) will not work, T is not mutable
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // increment `counter` 10 times - each time by different thread
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // `.lock` blocks the thread until mutex is unlocked
            let mut num = counter.lock().unwrap();
            *num += 1;
        }); // lock is dropped automatically at the end of block
        handles.push(handle);
    }

    // wait for all spawned threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

use std::sync::Barrier;
// barrier = stops main thread until 'n' threads arrive
fn run_barrier_example() {
    let mut handles = Vec::with_capacity(10);
    // 10 threads must arrive to `.wait()`
    let barrier = Arc::new(Barrier::new(10));

    for i in 0..10 {
        let c = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            println!("thread {} arrived to barrier", i);
            c.wait();
            println!("thread {} was released", i);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

use std::sync::RwLock;
fn run_rw_lock_example() {
    // 5 read locks, 1 write lock (if nobody reads)
    let lock = RwLock::new(5);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }
    {
        let mut w = lock.write().unwrap();
        // will cause deadlock, can use `try_read`
        // let r1 = lock.read().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    }
}

pub fn run_threads_demo() {
    compute_independent_tasks_with_basic_threading();
    println!();
    run_message_passing_between_threads_example();
    println!();
    run_mutex_example();
    println!();
    run_barrier_example();
    println!();
    run_rw_lock_example();
}
