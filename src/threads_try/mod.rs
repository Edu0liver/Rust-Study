
mod threads_try {
    use std::{thread, time::Duration};
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};

    fn ex_1(){
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
    
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    
        handle.join().unwrap();
    }

    fn ex_2(){
        let v = vec![1, 2, 3];
        
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);	
        });

        handle.join().unwrap();
    }

    fn ex_3(){
        let (tx, rx) = mpsc::channel();
        let tx2 = tx.clone();

        thread::spawn(move || {
            let val = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for v in val {
                tx.send(v).unwrap();
                thread::sleep(Duration::from_secs(1));
            }

        });

        thread::spawn(move || {
            let val = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for v in val {
                tx2.send(v).unwrap();
                thread::sleep(Duration::from_secs(1));
            }

        });

        for received in rx {
            println!("Got: {}", received);
        }
        
    }

    fn ex_4(){
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    fn ex_5(){
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
    
    pub fn threads_call() {
        ex_5();
    }

}

pub fn threads_try() {
    crate::threads_try::threads_try::threads_call();
}
