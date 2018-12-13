extern crate parking_lot;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use std::thread::{sleep, spawn};
use std::time::Duration;

static FLAG: AtomicBool = ATOMIC_BOOL_INIT;

fn main() {
    let lock = ParkingLot(parking_lot::RwLock::new(()));

    read_write_read(lock);
}

fn read_write_read<R, W>(lock: impl ReadWriteLock<R, W>) {
    let lock = Arc::new(lock);
    let cloned_lock = Arc::clone(&lock);

    spawn(move || {
        let _r1 = lock.read();
        println!("1st read lock acquired");

        println!("waiting flag");
        while !FLAG.load(Ordering::SeqCst) { }
        println!("2nd sleep");
        sleep(Duration::from_secs(1));

        println!("wait read lock");
        let _r2 = lock.read();
        println!("2nd read lock acquired");
    });

    println!("1st sleep");
    sleep(Duration::from_secs(1));
    println!("set flag");
    FLAG.store(true, Ordering::SeqCst);
    println!("wait write lock");
    let _w = cloned_lock.write();
    println!("write lock acquired");
}

trait ReadWriteLock<R, W>: Send + Sync {
    fn read(&self) -> R;
    fn write(&self) -> W;
}

struct ParkingLot(parking_lot::RwLock<()>);
impl<'a> ReadWriteLock<parking_lot::RwLockReadGuard<'a, ()>, parking_lot::RwLockWriteGuard<'a, ()>> for ParkingLot {
    fn read(&self) -> parking_lot::RwLockReadGuard<()> {
        self.0.read()
    }
    fn write(&self) -> parking_lot::RwLockWriteGuard<()> {
        self.0.write()
    }
}
unsafe impl Send for ParkingLot { }
unsafe impl Sync for ParkingLot { }
