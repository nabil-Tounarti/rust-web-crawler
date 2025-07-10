//
// Web Crawler Exercise in Rust
// Implement three different approaches to web crawling:
// 1. Serial crawler (sequential)
// 2. Concurrent crawler with shared state and Mutex
// 3. Concurrent crawler with channels
//

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;

// TODO: Define the Fetcher trait
// Hint: It should have a fetch method that takes a URL and returns a Result
trait Fetcher {
    // TODO: Define the fetch method signature
    fn fetch(&self, url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}

// TODO: Implement a FakeFetcher struct that simulates web fetching
// Hint: Use a HashMap to store fake results
struct FakeFetcher {
    // TODO: Add field to store fake data
    // data: HashMap<String, FakeResult>,
}

struct FakeResult {
    body: String,
    urls: Vec<String>,
}

impl Fetcher for FakeFetcher {
    // TODO: Implement the fetch method
    // - Check if URL exists in fake data
    // - Print "found: {url}" or "missing: {url}"
    // - Return urls or error appropriately
    fn fetch(&self, url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        todo!("Implement fetch method")
    }
}

// TODO: Create a function to build the fake fetcher with test data
// Hint: Use the same URLs and structure as the Go version
fn create_fake_fetcher() -> FakeFetcher {
    todo!("Create fake fetcher with test data")
}

//
// EXERCISE 1: Serial Crawler
//
// TODO: Implement a serial (non-concurrent) crawler
// Parameters:
// - url: the starting URL
// - fetcher: the fetcher implementation
// - fetched: a mutable reference to a HashMap tracking visited URLs
//
// Algorithm:
// 1. Check if URL is already fetched, return if so
// 2. Mark URL as fetched
// 3. Fetch URLs from the current URL
// 4. Recursively crawl each returned URL
fn serial_crawler(
    url: &str,
    fetcher: &dyn Fetcher,
    fetched: &mut HashMap<String, bool>,
) {
    todo!("Implement serial crawler")
}

//
// EXERCISE 2: Concurrent Crawler with Mutex
//
// TODO: Define a struct to hold shared state with mutex protection
struct FetchState {
    // TODO: Add mutex-protected HashMap
    // fetched: Arc<Mutex<HashMap<String, bool>>>,
}

impl FetchState {
    // TODO: Implement new() constructor
    fn new() -> Self {
        todo!("Create new FetchState")
    }

    // TODO: Implement test_and_set method
    // This should:
    // 1. Lock the mutex
    // 2. Check if URL was already fetched
    // 3. Mark URL as fetched
    // 4. Return whether it was already fetched
    fn test_and_set(&self, url: &str) -> bool {
        todo!("Implement test_and_set")
    }
}

// TODO: Implement concurrent crawler using mutex
// This should:
// 1. Use test_and_set to check/mark URLs atomically
// 2. Spawn threads for each new URL found
// 3. Use thread::spawn and join to wait for all threads
fn concurrent_mutex_crawler(
    url: &str,
    fetcher: Arc<dyn Fetcher + Send + Sync>,
    state: Arc<FetchState>,
) {
    todo!("Implement concurrent mutex crawler")
}

//
// EXERCISE 3: Concurrent Crawler with Channels
//
// TODO: Implement a worker function that fetches URLs and sends results via channel
async fn worker(
    url: String,
    fetcher: Arc<dyn Fetcher + Send + Sync>,
    sender: mpsc::UnboundedSender<Vec<String>>,
) {
    todo!("Implement worker function")
}

// TODO: Implement coordinator function that manages the crawling process
async fn coordinator(
    mut receiver: mpsc::UnboundedReceiver<Vec<String>>,
    fetcher: Arc<dyn Fetcher + Send + Sync>,
) {
    todo!("Implement coordinator function")
    // Hints:
    // - Keep track of number of active workers
    // - Maintain a HashMap of fetched URLs
    // - Spawn new workers for unfetched URLs
    // - Stop when no more workers are active
}

// TODO: Implement the main concurrent channel crawler function
async fn concurrent_channel_crawler(
    url: &str,
    fetcher: Arc<dyn Fetcher + Send + Sync>,
) {
    todo!("Implement concurrent channel crawler")
    // Hints:
    // - Create an unbounded channel
    // - Send initial URL to channel
    // - Start coordinator
}

//
// Main function and tests
//
#[tokio::main]
async fn main() {
    let fetcher = create_fake_fetcher();
    let fetcher_arc = Arc::new(fetcher);

    println!("=== Serial ===");
    // TODO: Call serial crawler
    
    println!("\n=== Concurrent Mutex ===");
    // TODO: Call concurrent mutex crawler
    
    println!("\n=== Concurrent Channel ===");
    // TODO: Call concurrent channel crawler
}

//
// BONUS EXERCISES:
//
// 1. Add error handling throughout the code
// 2. Implement proper Error types instead of using Box<dyn std::error::Error>
// 3. Add timing measurements to compare performance
// 4. Implement a depth limit for crawling
// 5. Add tests for each crawler implementation
//
// HINTS:
// - Use Arc<Mutex<T>> for shared mutable state
// - Use Arc<T> for shared immutable state
// - Use thread::spawn for the mutex version
// - Use tokio::spawn for the channel version
// - Remember to handle the case where fetching fails
// - The channel version needs careful management of the worker count
//