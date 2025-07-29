use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// The behavior of a fetcher.
trait Fetcher: Send + Sync + 'static {
    /// Fetch returns a list of URLs found on the page, or an error.
    fn fetch(&self, url: &str) -> Result<Vec<String>, String>;
}

/// A fake fetcher that returns canned results for testing.
/// It stores a Result to allow for simulating fetch errors.
struct FakeFetcher {
    results: HashMap<String, Result<Vec<String>, String>>,
}

impl Fetcher for FakeFetcher {
    fn fetch(&self, url: &str) -> Result<Vec<String>, String> {
        println!("Fetching: {url}");
        // Simulate network latency to make concurrency benefits more apparent.
        thread::sleep(Duration::from_millis(100));
        match self.results.get(url) {
            Some(Ok(urls)) => {
                println!("Found:    {url}");
                Ok(urls.clone())
            }
            Some(Err(e)) => {
                println!("Error on: {url}");
                Err(e.clone())
            }
            None => {
                println!("Missing:  {url}");
                Err(format!("not found: {url}"))
            }
        }
    }
}

fn get_fake_fetcher() -> FakeFetcher {
    let mut results = HashMap::new();
    results.insert(
        "https://example.com/".to_string(),
        Ok(vec![
            "https://example.com/page1".to_string(),
            "https://example.com/page2".to_string(),
        ]),
    );
    results.insert(
        "https://example.com/page1".to_string(),
        Ok(vec![
            "https://example.com/".to_string(),
            "https://example.com/page3".to_string(),
        ]),
    );
    results.insert(
        "https://example.com/page2".to_string(),
        Ok(vec!["https://example.com/".to_string()]),
    );
    results.insert(
        "https://example.com/page3".to_string(),
        Ok(vec![
            "https://example.com/page1".to_string(),
            "https://example.com/page4".to_string(),
        ]),
    );
    results.insert("https://example.com/page4".to_string(), Ok(vec![]));

    FakeFetcher { results }
}

//===========================================================================
// Part 1: Serial Crawler (User's Implementation)
//===========================================================================

fn serial_crawler(url: String, fetcher: &impl Fetcher, fetched: &mut HashSet<String>) {
    if fetched.contains(&url) {
        return;
    }
    fetched.insert(url.clone());
    if let Ok(fetched_urls) = fetcher.fetch(&url) {
        for url in fetched_urls {
            serial_crawler(url, fetcher, fetched);
        }
    }
}

//===========================================================================
// Part 2: Concurrent Crawler with Mutex (User's Implementation)
//===========================================================================

fn concurrent_mutex_crawler(
    url: String,
    fetcher: Arc<impl Fetcher>,
    fetched: Arc<Mutex<HashSet<String>>>,
) {
    {
        let mut cache = fetched.lock().unwrap();
        if cache.contains(&url) {
            return;
        }
        cache.insert(url.clone());
    }
    let mut threads = vec![];
    if let Ok(fetched_urls) = fetcher.fetch(&url) {
        for url in fetched_urls {
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

//===========================================================================
// Part 3: Concurrent Crawler with Channels (Corrected Implementation)
//===========================================================================

/// The concurrent crawler using channels for communication.
/// This version is corrected to handle termination properly.
fn concurrent_channel_crawler(url: String, fetcher: Arc<impl Fetcher>) -> HashSet<String> {
    // The channel will transport the result of a fetch operation.
    let (tx, rx) = mpsc::channel::<Result<Vec<String>, String>>();
    let mut fetched = HashSet::new();
    let mut outstanding_fetches = 0;

    // Start the first fetch operation.
    if !fetched.contains(&url) {
        fetched.insert(url.clone());
        outstanding_fetches += 1;
        let tx_initial = tx.clone();
        let fetcher_initial = Arc::clone(&fetcher);
        thread::spawn(move || {
            tx_initial.send(fetcher_initial.fetch(&url)).unwrap();
        });
    }

    // This is the coordinator loop. It receives results and dispatches new work.
    while outstanding_fetches > 0 {
        // Block until a worker sends a result.
        let result = rx.recv().unwrap();
        outstanding_fetches -= 1;

        if let Ok(urls) = result {
            for u in urls {
                // If we find a new, unfetched URL...
                if !fetched.contains(&u) {
                    fetched.insert(u.clone());
                    outstanding_fetches += 1; // ...increment our counter...

                    // ...and spawn a new worker to fetch it.
                    let tx_worker = tx.clone();
                    let fetcher_worker = Arc::clone(&fetcher);
                    thread::spawn(move || {
                        tx_worker.send(fetcher_worker.fetch(&u)).unwrap();
                    });
                }
            }
        }
    }
    fetched
}

//===========================================================================
// Main Function to run the examples (User's Implementation)
//===========================================================================

fn main() {
    let fetcher = get_fake_fetcher();
    let fetcher_arc = Arc::new(fetcher);

    println!("--- Serial Crawler ---");
    let mut fetched_serial = HashSet::new();
    serial_crawler(
        "https://example.com/".to_string(),
        &*fetcher_arc,
        &mut fetched_serial,
    );
    println!("----------------------\n");

    println!("--- Concurrent Mutex Crawler ---");
    let fetched_mutex = Arc::new(Mutex::new(HashSet::new()));
    concurrent_mutex_crawler(
        "https://example.com/".to_string(),
        fetcher_arc.clone(),
        fetched_mutex,
    );
    println!("----------------------\n");

    println!("--- Concurrent Channel Crawler ---");
    concurrent_channel_crawler("https://example.com/".to_string(), fetcher_arc);
    println!("----------------------\n");
}

//===========================================================================
// Test Suite
//===========================================================================

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
        concurrent_mutex_crawler(
            "https://example.com/".to_string(),
            fetcher,
            Arc::clone(&fetched_arc),
        );

        let fetched_guard = fetched_arc.lock().unwrap();
        let expected = get_expected_urls();
        assert_eq!(*fetched_guard, expected);
    }

    #[test]
    fn test_concurrent_channel_crawler_happy_path() {
        let fetcher = Arc::new(get_fake_fetcher());
        let fetched = concurrent_channel_crawler("https://example.com/".to_string(), fetcher);

        let expected = get_expected_urls();
        assert_eq!(fetched, expected);
    }

    #[test]
    fn test_start_with_nonexistent_url() {
        let fetcher = get_fake_fetcher();
        let url = "https://nonexistent.com/".to_string();
        let expected: HashSet<String> = [url.clone()].iter().cloned().collect();

        // Serial
        let mut fetched_serial = HashSet::new();
        serial_crawler(url.clone(), &fetcher, &mut fetched_serial);
        assert_eq!(
            fetched_serial, expected,
            "Serial crawler failed on non-existent URL"
        );

        // Mutex
        let fetcher_arc = Arc::new(get_fake_fetcher()); // Re-create Arc for this test
        let fetched_mutex = Arc::new(Mutex::new(HashSet::new()));
        concurrent_mutex_crawler(url.clone(), fetcher_arc.clone(), fetched_mutex.clone());
        assert_eq!(
            *fetched_mutex.lock().unwrap(),
            expected,
            "Mutex crawler failed on non-existent URL"
        );

        // Channel
        let fetched_channel = concurrent_channel_crawler(url.clone(), fetcher_arc);
        assert_eq!(
            fetched_channel, expected,
            "Channel crawler failed on non-existent URL"
        );
    }

    #[test]
    fn test_single_url_no_links() {
        let mut results = HashMap::new();
        let url = "https://single.com/".to_string();
        results.insert(url.clone(), Ok(vec![]));
        let fetcher = FakeFetcher { results };
        let expected: HashSet<String> = [url.clone()].iter().cloned().collect();

        // Serial
        let mut fetched_serial = HashSet::new();
        serial_crawler(url.clone(), &fetcher, &mut fetched_serial);
        assert_eq!(
            fetched_serial, expected,
            "Serial crawler failed on single URL"
        );

        // Mutex
        let fetcher_arc = Arc::new(fetcher);
        let fetched_mutex = Arc::new(Mutex::new(HashSet::new()));
        concurrent_mutex_crawler(url.clone(), fetcher_arc.clone(), fetched_mutex.clone());
        assert_eq!(
            *fetched_mutex.lock().unwrap(),
            expected,
            "Mutex crawler failed on single URL"
        );

        // Channel
        let fetched_channel = concurrent_channel_crawler(url.clone(), fetcher_arc);
        assert_eq!(
            fetched_channel, expected,
            "Channel crawler failed on single URL"
        );
    }

    #[test]
    fn test_crawler_with_fetch_error() {
        let mut results = HashMap::new();
        let start_url = "https://start.com/".to_string();
        let ok_url = "https://ok.com/".to_string();
        let bad_url = "https://bad.com/".to_string();
        results.insert(start_url.clone(), Ok(vec![ok_url.clone(), bad_url.clone()]));
        results.insert(ok_url.clone(), Ok(vec![]));
        results.insert(bad_url.clone(), Err("permanent failure".to_string()));
        let fetcher = FakeFetcher { results };

        // The crawlers should attempt to fetch all three URLs, even if one fails.
        let expected: HashSet<String> = [start_url.clone(), ok_url.clone(), bad_url.clone()]
            .iter()
            .cloned()
            .collect();

        // Serial
        let mut fetched_serial = HashSet::new();
        serial_crawler(start_url.clone(), &fetcher, &mut fetched_serial);
        assert_eq!(
            fetched_serial, expected,
            "Serial crawler failed with fetch error"
        );

        // Mutex
        let fetcher_arc = Arc::new(fetcher);
        let fetched_mutex = Arc::new(Mutex::new(HashSet::new()));
        concurrent_mutex_crawler(
            start_url.clone(),
            fetcher_arc.clone(),
            fetched_mutex.clone(),
        );
        assert_eq!(
            *fetched_mutex.lock().unwrap(),
            expected,
            "Mutex crawler failed with fetch error"
        );

        // Channel
        let fetched_channel = concurrent_channel_crawler(start_url.clone(), fetcher_arc);
        assert_eq!(
            fetched_channel, expected,
            "Channel crawler failed with fetch error"
        );
    }
}
