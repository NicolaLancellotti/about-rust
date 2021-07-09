use std::sync::*;
use std::thread;
use std::time::Duration;

/*
    Send: Types that can be transferred across thread boundaries.
    Sync: Types for which it is safe to share references between threads.
*/

#[test]
fn threads() {
    let box1: Box<i32> = Box::new(10);

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        println!("value: {box1}");
    });

    let r = handle.join();
}

#[test]
fn message_passing() {
    enum Message {
        Item(Box<i32>),
    }

    //  Multiple producer, single consumer.
    let (sender1, receiver) = mpsc::channel();
    let sender2 = sender1.clone();
    let receiver1 = Arc::new(Mutex::new(receiver));
    let receiver2 = receiver1.clone();

    fn make_producer(
        sender: std::sync::mpsc::Sender<Message>,
        name: String,
        start_value: i32,
    ) -> std::thread::JoinHandle<()> {
        thread::spawn(move || {
            for i in start_value..(start_value + 2) {
                thread::sleep(Duration::from_millis(300));
                println!("{name} - {i}");
                let data = Message::Item(Box::new(i));
                sender.send(data).unwrap();
            }
        })
    }

    fn make_consumer(
        receiver: Arc<Mutex<std::sync::mpsc::Receiver<Message>>>,
        name: String,
    ) -> std::thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                match receiver.lock().unwrap().recv() {
                    Ok(Message::Item(value)) => println!("{name} - {value}"),
                    Err(_) => break, // Channel closed,
                }
            }
        })
    }
    let producer1 = make_producer(sender1, "P1".to_string(), 0);
    let producer2 = make_producer(sender2, "P2".to_string(), 2);
    let consumer1 = make_consumer(receiver1, "C1".to_string());
    let consumer2 = make_consumer(receiver2, "C1".to_string());
    let _ = producer1.join();
    let _ = producer2.join();
    let _ = consumer1.join();
    let _ = consumer2.join();
}

#[test]
fn condvar() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        thread::sleep(::std::time::Duration::from_secs(2));
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    while !*started {
        started = cvar.wait(started).unwrap();
    }
}

#[test]
fn once_lock() {
    static LOCK: OnceLock<usize> = OnceLock::new();
    std::thread::spawn(|| {
        let value = LOCK.get_or_init(|| 1);
        assert_eq!(value, &1);
    })
    .join()
    .unwrap();

    assert_eq!(LOCK.get(), Some(&1),);
}

#[test]
fn lazy_lock() {
    static LOCK: LazyLock<usize> = LazyLock::new(|| 1);
    std::thread::spawn(|| {
        assert_eq!(*LOCK, 1);
    })
    .join()
    .unwrap();

    assert_eq!(*LOCK, 1);
}
