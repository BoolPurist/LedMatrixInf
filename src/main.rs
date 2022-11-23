extern crate redis;
use redis::Commands;

fn main() {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/").expect("Could not connect to");
    let mut con = client.get_connection().expect("Could not get connection");
    // throw away the result, just make sure it does not fail
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let value: String = con
        .get("mykey")
        .expect("Could not extract value from key mykey");

    println!("{}", value)
}
