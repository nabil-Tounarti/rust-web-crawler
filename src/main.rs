use std::collections::{HashMap, HashSet};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// The behavior of a fetcher.
trait Fetcher: Send + Sync + 'static {
    // Fetch returns a slice of URLs found on the page.
    fn fetch(&self, url: &str) -> Result<Vec<String>, String>;
}

// A fake fetcher that returns canned results.
// We use a HashMap to store the fake data.
struct FakeFetcher {
    results: HashMap<String, Vec<String>>,
}

// Implement the Fetcher trait for our FakeFetcher.
impl Fetcher for FakeFetcher {
    fn fetch(&self, url: &str) -> Result<Vec<String>, String> {
        println!("Fetching: {}", url);
        thread::sleep(Duration::from_millis(500)); // Simulate network latency
        match self.results.get(url) {
            Some(urls) => {
                println!("Found:    {}", url);
                Ok(urls.clone())
            }
            None => {
                println!("Missing:  {}", url);
                Err(format!("not found: {}", url))
            }
        }
    }
}

// Helper function to create a populated FakeFetcher.
fn get_fake_fetcher() -> FakeFetcher {
    let mut results = HashMap::new();
    results.insert(
        "https://example.com/".to_string(),
        vec![
            "https://example.com/page1".to_string(),
            "https://example.com/page2".to_string(),
        ],
    );
    results.insert(
        "https://example.com/page1".to_string(),
        vec![
            "https://example.com/".to_string(),
            "https://example.com/page3".to_string(),
        ],
    );
    results.insert(
        "https://example.com/page2".to_string(),
        vec!["https://example.com/".to_string()],
    );
    results.insert(
        "https://example.com/page3".to_string(),
        vec![
            "https://example.com/page1".to_string(),
            "https://example.com/page4".to_string(),
        ],
    );
    results.insert("https://example.com/page4".to_string(), vec![]);

    FakeFetcher { results }
}
 fn main(){
    print!("web crawler !")
 }