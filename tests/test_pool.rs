extern crate rustorm;
use rustorm::database::Pool;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_pool(){
    let mut pool = Pool::init();
    assert_eq!(pool.total_free_connections(), 0);
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    pool.reserve_connection(&url, 5);
    assert_eq!(pool.total_free_connections(), 5);
}

//#[test]
fn test_max_pool(){
    let mut pool = Pool::init();
    assert_eq!(pool.total_free_connections(), 0);
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    pool.reserve_connection(&url, 90);
    assert_eq!(pool.total_free_connections(), 90);
}

#[test]
fn test_use(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    pool.reserve_connection(&url, 5);
    assert_eq!(pool.total_free_connections(), 5);
    pool.get_db_with_url(&url);
}


#[test]
fn test_use_no_reserve(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    assert_eq!(pool.total_free_connections(), 0);
    pool.get_db_with_url(&url);
}

#[test]
fn test_arc_mutex_connection(){
    let mut pool = Arc::new(Mutex::new(Pool::init()));
    for i in 0..10{
        let pool = pool.clone();
        thread::spawn( move || {
            let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
            let mut pool = pool.lock().unwrap();
            //println!("index[{}] has {} connections..", i, pool.total_pool_connection());
            let db = pool.get_db_with_url(&url);
        });
    }
    
}



#[test]
fn test_release_connection(){
    let mut pool = Pool::init();
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    println!("{} connections..", pool.total_free_connections());
    let db = pool.get_db_with_url(&url);
    pool.release(db.unwrap());
    
}