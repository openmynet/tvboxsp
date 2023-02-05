# Quick Network Scanner Library

Rust library for scanning network hosts asynchronously.

Currently, the following scan modes are supported:

* TCP Connect;
* Ping (ICMP Echo / Echo Reply).

> NOTE: in order to properly use the library you may need to increase the
> maximum allowed open files. E.g.:

```bash
ulimit -n 10000
```

> NOTE: for the ping scan mode, you need `root` or other
> proper permissions (i.e. CAP_NET_RAW).

See the library on [crates.io](https://crates.io/crates/qscan).

## Usage

Dependencies (`Cargo.toml`):

```bash
[dependencies]
qscan = "0.6.0"
tokio = { version = "1", features = ["rt-multi-thread"] }
```

Alternatively, in order enable json serialization of results structures,
activate `serialize` feature:

```bash
[dependencies]
qscan = { version = "0.6.0" , features = ["serialize"] }
tokio = { version = "1", features = ["rt-multi-thread"] }
```

and then (`src/main.rs`):

### From [TCP connect scan example](./examples/scan_tcp_connect.rs)

```rust
use qscan::{QSPrintMode, QScanResult, QScanTcpConnectState, QScanType, QScanner};
use tokio::runtime::Runtime;

pub fn main() {
    let mut scanner = QScanner::new("8.8.8.8,127.0.0.1", "53,80,443");
    scanner.set_batch(5000);
    scanner.set_timeout_ms(2000);
    scanner.set_ntries(1);
    scanner.set_scan_type(QScanType::TcpConnect);
    scanner.set_print_mode(QSPrintMode::NonRealTime);

    let res: &Vec<QScanResult> = Runtime::new().unwrap().block_on(scanner.scan_tcp_connect());

    for r in res {
        if let QScanResult::TcpConnect(sa) = r {
            if sa.state == QScanTcpConnectState::Open {
                println!("{}", sa.target);
            }
        }
    }
}
```

See also the [provided ping example](./examples/scan_ping.rs) and [qsc
utility](../qsc/).
