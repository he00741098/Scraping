use std::ffi::OsStr;
use std::path::Path;
use tokio::fs::read_dir;

#[tokio::main]
fn main() {
    let path = Path::new("./json/");
    let dir = read_dir(path).await.expect("Path exists");
    let mut entries_read = 0;
    while let Ok(Some(entry)) = dir.next_entry() {
        
        entries_read++;
    }
    println!("Read {} Entries!", entries_read);
}
