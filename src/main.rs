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

fn serial_crawler(url: String, fetcher: &impl Fetcher, fetched: &mut HashSet<String>) {
    if fetched.contains(&url) {
        return
    } 
    fetched.insert(url.clone());
    let fetched_urls = fetcher.fetch(&url).unwrap_or_default();
    for url in  fetched_urls {
        serial_crawler(url, fetcher, fetched);
    }
}

fn concurrent_mutex_crawler(url: String, fetcher: Arc<impl Fetcher>, fetched: Arc<Mutex<HashSet<String>>>) {
    {
        let mut cache = fetched.lock().unwrap();
        if cache.contains(&url) {
            return
        } 
        cache.insert(url.clone());
    }
    let mut threads = vec![];
    if let Ok(fetched_urls) = fetcher.fetch(&url){
        
        for url in  fetched_urls {
            let clone_fetcher = fetcher.clone();
            let fetched_clone = fetched.clone();
            threads.push(thread::spawn(move || {
                concurrent_mutex_crawler(url, clone_fetcher, fetched_clone);
            }));
        }
    }
    for thread in threads {
        thread.join().unwrap();
    }
}

 fn main() {
    let fetcher = get_fake_fetcher();
    let fetcher_arc = Arc::new(fetcher);

    println!("--- Serial Crawler ---");
    let mut fetched_serial = HashSet::new();
    serial_crawler("https://example.com/".to_string(), &*fetcher_arc, &mut fetched_serial);
    println!("----------------------\n");

    println!("--- Concurrent Mutex Crawler ---");
    let fetched_mutex = Arc::new(Mutex::new(HashSet::new()));
    concurrent_mutex_crawler("https://example.com/".to_string(), fetcher_arc.clone(), fetched_mutex);
    println!("----------------------\n");

    // println!("--- Concurrent Channel Crawler ---");
    // concurrent_channel_crawler("https://example.com/".to_string(), fetcher_arc);
    // println!("----------------------\n");
}



 #[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Helper function to get the set of all URLs we expect to be crawled.
    fn get_expected_urls() -> HashSet<String> {
        [
            "https://example.com/",
            "https://example.com/page1",
            "https://example.com/page2",
            "https://example.com/page3",
            "https://example.com/page4",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_serial_crawler_happy_path() {
        let fetcher = get_fake_fetcher();
        let mut fetched = HashSet::new();
        serial_crawler("https://example.com/".to_string(), &fetcher, &mut fetched);
        
        let expected = get_expected_urls();
        assert_eq!(fetched, expected);
    }
  
    #[test]
    fn test_concurrent_mutex_crawler_happy_path() {
        let fetcher = Arc::new(get_fake_fetcher());
        let fetched_arc = Arc::new(Mutex::new(HashSet::new()));
        concurrent_mutex_crawler("https://example.com/".to_string(), fetcher, Arc::clone(&fetched_arc));

        let fetched_guard = fetched_arc.lock().unwrap();
        let expected = get_expected_urls();
        assert_eq!(*fetched_guard, expected);
    }
}
