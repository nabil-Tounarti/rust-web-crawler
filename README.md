# Rust Web Crawler

A high-performance, concurrent web crawler implemented in Rust that demonstrates different approaches to web crawling with varying levels of concurrency and synchronization.

## 🚀 Features

- **Multiple Crawling Strategies**: Three different implementations showcasing various concurrency patterns
- **Thread-Safe Design**: Proper synchronization using mutexes and channels
- **Extensible Architecture**: Trait-based fetcher interface for easy testing and extension
- **Comprehensive Testing**: Full test suite covering edge cases and error scenarios
- **Performance Optimized**: Concurrent implementations for improved crawling speed

## 📋 Project Structure

```
rust_web_crawler/
├── src/
│   └── main.rs          # Main implementation with three crawler variants
├── Cargo.toml           # Project dependencies and metadata
├── Cargo.lock           # Locked dependency versions
└── README.md           # This file
```

## 🔧 Implementation Approaches

### 1. Serial Crawler
- **File**: `serial_crawler()` function
- **Approach**: Sequential, depth-first traversal
- **Use Case**: Simple crawling, debugging, or when order matters
- **Performance**: Slower but predictable and easy to understand

### 2. Concurrent Mutex Crawler
- **File**: `concurrent_mutex_crawler()` function
- **Approach**: Multi-threaded with shared state protected by mutex
- **Use Case**: When you need fine-grained control over shared state
- **Performance**: Good concurrency with thread safety

### 3. Concurrent Channel Crawler
- **File**: `concurrent_channel_crawler()` function
- **Approach**: Worker pool pattern using channels for communication
- **Use Case**: High-performance crawling with better resource management
- **Performance**: Optimal concurrency with reduced contention

## 🛠️ Architecture

### Core Components

#### Fetcher Trait
```rust
trait Fetcher: Send + Sync + 'static {
    fn fetch(&self, url: &str) -> Result<Vec<String>, String>;
}
```
- Defines the interface for fetching URLs
- Returns a list of discovered URLs or an error
- Thread-safe and can be shared across threads

#### FakeFetcher
- Test implementation that simulates web crawling
- Pre-defined responses for consistent testing
- Simulates network latency for realistic performance testing

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ (edition 2024)
- Cargo package manager

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/nabil-Tounarti/rust-web-crawler.git
   cd rust-web-crawler
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run the crawler**
   ```bash
   cargo run
   ```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_serial_crawler_happy_path
```

## 📊 Performance Comparison

The project demonstrates three different crawling approaches:

| Approach | Concurrency | Thread Safety | Performance | Complexity |
|----------|-------------|---------------|-------------|------------|
| Serial | None | N/A | Slow | Low |
| Mutex | High | Mutex-protected | Medium | Medium |
| Channel | High | Channel-based | Fast | High |

## 🧪 Testing

The project includes comprehensive tests covering:

- **Happy Path**: Basic crawling functionality
- **Edge Cases**: Non-existent URLs, fetch errors
- **Concurrency**: Thread safety and race condition prevention
- **Error Handling**: Proper error propagation and handling

### Test Categories

1. **Basic Functionality Tests**
   - `test_serial_crawler_happy_path`
   - `test_concurrent_mutex_crawler_happy_path`
   - `test_concurrent_channel_crawler_happy_path`

2. **Error Handling Tests**
   - `test_start_with_nonexistent_url`
   - `test_crawler_with_fetch_error`

3. **Edge Case Tests**
   - `test_single_url_no_links`

## 🔄 Extending the Project

### Adding Real HTTP Fetcher

To implement a real web crawler, create a new fetcher:

```rust
use reqwest;

struct HttpFetcher {
    client: reqwest::Client,
}

impl Fetcher for HttpFetcher {
    fn fetch(&self, url: &str) -> Result<Vec<String>, String> {
        // Implement real HTTP fetching logic
        // Parse HTML and extract links
        // Return discovered URLs
    }
}
```

### Adding Configuration

Extend the project with configuration options:

```rust
struct CrawlerConfig {
    max_depth: usize,
    max_concurrent_requests: usize,
    request_timeout: Duration,
    user_agent: String,
}
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🎯 Learning Objectives

This project serves as an excellent learning resource for:

- **Rust Concurrency**: Understanding threads, mutexes, and channels
- **Trait-based Design**: Building extensible and testable code
- **Error Handling**: Proper error propagation in Rust
- **Testing**: Comprehensive test coverage and mocking
- **Performance Optimization**: Comparing different concurrency approaches

## 🔗 Dependencies

- **tokio**: Async runtime for high-performance networking (currently unused but ready for HTTP implementation)
- **Standard Library**: Threading, collections, and synchronization primitives

## 📈 Future Enhancements

- [ ] Real HTTP fetcher implementation
- [ ] Rate limiting and politeness controls
- [ ] URL filtering and domain restrictions
- [ ] Persistent storage for crawled data
- [ ] Configuration file support
- [ ] Metrics and monitoring
- [ ] Distributed crawling capabilities

---

**Note**: This is currently a demonstration project using a fake fetcher. For production use, implement a real HTTP fetcher with proper error handling and rate limiting. 