
mod threads_try {
    use std::{thread, time::Duration};

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
    
    pub fn threads_call() {
        ex_1()
    }

}

pub fn threads_try() {
    crate::threads_try::threads_try::threads_call();
}
