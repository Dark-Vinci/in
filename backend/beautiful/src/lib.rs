// use std::rc::{Rc, Weak};

// pub fn server() {
//     // usecase 1
//     enum List {
//         Cons(i32, Rc<List>),
//         Nil,
//     }

//     use List::*;

//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     let b = Cons(3, Rc::clone(&a));
//     let c = Cons(4, Rc::clone(&a));

//     // usecase 2
//     struct Node<T> {
//         val: T,
//         parent: Option<Weak<Node<T>>>,
//         left: Option<Rc<Node<T>>>,
//         right: Option<Rc<Node<T>>>,
//     }

//     impl<T> Node<T> {
//         fn new(val: T) -> Self {
//             Self { val, right: None, left: None, parent: None}
//         }

//         fn set_left(&mut self, l: Rc<Node<T>>) {
//             self.left = Some(l);
//         }

//         fn set_right(&mut self, r: Rc<Node<T>>) {
//             self.right = Some(r);
//         }

//         fn set_parent(&mut self, p: Weak<Node<T>>) {
//             self.parent = Some(p);
//         }
//     }

//     let  mut a = Rc::new(Node::new(1));
//     let mut b = Rc::new(Node::new(1));
//     let mut c = Rc::new(Node::new(1));

//     let weak_a = Rc::downgrade(&a);
//     Rc::clone(&b).set_parent(weak_a.clone());
//     Rc::clone(&c).set_parent(weak_a);

//     Rc::clone(&a).set_left(Rc::clone(&b));
//     Rc::clone(&a).set_right(Rc::clone(&c));


//     print!("{a}{b}{c}");
    

//     // let  mut a = Rc::new(Node::new(1));/
//     // let mut b = Rc::new(Node::new(1));
//     // let mut c = Rc::new(Node::new(1));

//     // a.clone().set_left(b.clone());

//     // Rc::clone(&mut b).set_parent(Rc::clone(&a));
//     // Rc::clone(&mut c).set_parent(Rc::clone(&a));
//     // Rc::clone(&mut a).set_left(Rc::clone(&b));
//     // Rc::clone(&mut a).set_right(Rc::clone(&c));


//     // c.parent = Some(Rc::clone(&a));
//     // b.parent = Some(Rc::clone(&a));

//     // let a1 = Rc::clone(&a);
//     // let a2 = Rc::clone(&b);
//     // let a3 = Rc::clone(&c);

//     // a.left = Some(a2);
//     // a.right = Some(a3);

//     // a.left = Some(Rc::clone(&r));




    
// }

use std::sync::{mpsc, Mutex, RwLock};
use std::sync::{atomic::AtomicBool, Arc};
use std::thread;

pub fn bat() {
    let a = Arc::new(Mutex::new(0));

    let mut handle = vec![];

    for i in 0..=100 {
        let c = Arc::clone(&a);

        let t = thread::spawn(move || {
            
            let mut value = c.lock().unwrap();

            *value += i;
        });

        handle.push(t)
    }

    for h in handle {
        h.join().unwrap();
    }

    println!("Final Value: {:?}", a);


}

pub fn abc() {
    // usecase for mutex
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    for _ in 0..=10 {
        let (data, tx) = (Arc::clone(&data), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();

            *data += 1;

            if *data == 10 {
                tx.send(()).unwrap();
            }
        });
    }

    rx.recv().unwrap();


    // Lock poisoning
    let lock = Arc::new(Mutex::new(0_u32));
    let lock2 = Arc::clone(&lock);

    let _ = thread::spawn(move || -> () {
        let _guard = lock2.lock().unwrap();

        // panic to poison the lock
        panic!();
    }).join();

    // The lock is poisoned at this point, we try to recover and get the underlining data
    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *guard += 1;



    let rdata = Arc::new(RwLock::new(1));

    let mut handles = vec![];

    for i in 0..=100 {
        let data = Arc::clone(&rdata);

        let t = thread::spawn(move || -> () {
            if i < 90 {
                let data = data.read().unwrap();

                println!("Thread {i} reads data {:?}", data);
            } else {
                let mut data = data.write().unwrap();
                *data += i;
            }
        });

        handles.push(t)
    }

    for t in handles {
        t.join().unwrap();
    }

    println!("The final value of the locked data is {:?}", rdata);
}
