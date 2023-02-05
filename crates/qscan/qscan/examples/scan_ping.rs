//
// qscan
// Copyright (C) 2022  0xor0ne
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
// PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//
//
use qscan::{QSPrintMode, QScanPingState, QScanResult, QScanTcpConnectState, QScanType, QScanner};
use std::net::IpAddr;
use tokio::runtime::Runtime;

pub fn main() {
    let mut scanner = QScanner::new("8.8.8.8,127.0.0.1,1.2.3.4", "");
    scanner.set_batch(5000);
    scanner.set_timeout_ms(2000);

    println!("Ping scan:");

    scanner.set_ping_interval_ms(500);
    scanner.set_ntries(2);
    scanner.set_scan_type(QScanType::Ping);
    scanner.set_print_mode(QSPrintMode::NonRealTime);

    let res: &Vec<QScanResult> = Runtime::new().unwrap().block_on(scanner.scan_ping());

    let mut ips_up: Vec<IpAddr> = Vec::new();

    for r in res {
        if let QScanResult::Ping(pr) = r {
            match pr.state {
                QScanPingState::Up => {
                    println!("  {}:UP", pr.target);
                    ips_up.push(pr.target);
                }
                QScanPingState::Down => {
                    println!("  {}:DOWN", pr.target);
                }
            }
        }
    }

    println!("Port scan targetting targets in UP state:");

    scanner.set_scan_type(QScanType::TcpConnect);
    scanner.set_vec_targets_addr(ips_up);
    scanner.set_targets_port("53,80,443,666");

    let res: &Vec<QScanResult> = Runtime::new().unwrap().block_on(scanner.scan_tcp_connect());

    for r in res {
        if let QScanResult::TcpConnect(sa) = r {
            if sa.state == QScanTcpConnectState::Open {
                println!("  {}:OPEN", sa.target);
            }

            if sa.state == QScanTcpConnectState::Close {
                println!("  {}:CLOSE", sa.target);
            }
        }
    }
}
