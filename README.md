# Rusty Port Scanner 🚀

A high-performance, concurrent port scanner written in Rust that supports TCP Connect scanning methods. This tool provides fast and efficient port scanning with service detection capabilities.

![Port Scanner Demo](https://github.com/Gamawn/portscan/blob/main/media/usage.gif)

## ✨ Features

- **Fast & Concurrent**: Utilizes async Rust for maximum performance
- **Multiple Scan Types**:
  - 🔌 TCP Connect Scanning (fallback mode)
- **Service Detection**: Identifies common services on open ports
- **Progress Visualization**: Real-time scanning progress with dual progress bars
- **Customizable**:
  - Worker count
  - Batch size
  - Timeouts
  - Port ranges
- **Resource Efficient**: Optimized for minimal system impact

## 🚀 Performance

- Full port scan (65535 ports) in seconds depending on network conditions
- Efficient memory usage with batch processing
- Concurrent scanning with customizable worker count

## 📋 Prerequisites

- Rust 1.70 or higher
- Linux/Unix or Windows operating system

## 🛠 Installation

1. Clone the repository:
```bash
git clone https://github.com/Gamawn/portscan.git
cd portscan
```

2. Build the project:
```bash
cargo build --release
```

The binary will be available at `target/release/portscan`

## 💻 Usage

### Basic Usage

```bash
# Regular TCP Connect scan
cargo run --release -- -i 192.168.1.1
```

### Advanced Usage

```bash
# Full scan with custom settings
cargo run --release -- \
    -i 192.168.1.1 \     # Target IP
    -s 1 \               # Start port
    -e 65535 \           # End port
    -w 1000 \            # Number of workers
    -b 500 \             # Batch size
    -m 100               # Timeout in ms
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `-i, --ip` | Target IP address | Required |
| `-s, --start` | Start port number | 1 |
| `-e, --end` | End port number | 1024 |
| `-m, --timeout` | Timeout in milliseconds | 200 |
| `-w, --workers` | Number of concurrent workers | CPU cores * 16 |
| `-b, --batch` | Batch size for processing | 100 |

## 📊 Output Example

```
Scanning 1024 ports on 192.168.1.1 with 64 workers (batch size: 100)

[00:00:02] ██████████████████████████████ 10/10 batches (5.0/s)
[00:00:02] ███████████░░░░░░░░░░░░░░░░░░░ 3 open ports found

Results:
PORT      STATE          SERVICE        LATENCY
--------------------------------------------------
22        open           SSH            12.5ms
80        open           HTTP           15.2ms
443       open           HTTPS          14.8ms

Scan completed!
Found 3 open ports
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📝 TODO

- [ ] Add SYN functionality for Linux
- [ ] Add UDP scanning support
- [ ] Implement service version detection
- [ ] Add export functionality (JSON, CSV)
- [ ] Improve Windows support for SYN scanning
- [ ] Add configuration file support
- [ ] Implement more advanced scan patterns

## 🙏 Acknowledgments

- [tokio](https://tokio.rs/) for the async runtime
- [clap](https://clap.rs/) for CLI argument parsing
- [indicatif](https://github.com/console-rs/indicatif) for progress bars

## 💡 Tips

1. For fastest scanning:
   - Increase worker count (-w option)
   - Decrease timeout (-m option)
   - Increase batch size (-b option)

2. For more reliable scanning:
   - Use TCP Connect scanning
   - Keep default timeout
   - Reduce worker count
   - Use smaller batch sizes
