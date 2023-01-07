use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;

fn main() {
    // create a global variable
    lazy_static! {
        static ref FRUITS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    }

    // add new fruit to the global variable by gettin a lock on the Mutex
    fn insert(fruit: &str) {
        let mut data = FRUITS.lock().unwrap();
        data.push(fruit.to_string());
    }

    insert("Apple");
    insert("Banana");
    insert("Cherry");

    {
        // create another thread that tries to modify the global variable
        let t = thread::spawn(move || {
            insert("Mango");
        });

        // wait for the other thread to finish
        t.join().unwrap();

        let db = FRUITS
            .lock()
            .map_err(|_| println!("Failed to a get a guard on the Mutex value"))
            .unwrap();

        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("{}: {}", i, item));

        // Long but clearer
        // let enumerator = db.iter().enumerate();
        // for (i, fruit) in enumerator {
        //     println!("Fruit: {}{}", i, fruit);
        // }

        // creates a race condition
        let t = thread::spawn(move || {
            insert("Peach");
        });

        // wait for the other thread to finish
        t.join().unwrap();
    }

    insert("Smart Fruit");
}
