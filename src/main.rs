extern crate redis;
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    let _ : () = con.set("my_key", 42)?;
    con.get("my_key")
}

fn main() {
    println!("The redis result is: {}", fetch_an_integer().unwrap_or(0));
}
