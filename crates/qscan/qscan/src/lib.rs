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

//! # QSCAN
//!
//! Asynchronous network ports scanning library
//!
//! To enable `qscan::QScanTcpConnectResult` serialization:
//!
//! ```text
//! [dependencies]
//! qscan = { path = "../qscan", version = "0.5.0" , features = ["serialize"] }
//! ```

pub use crate::qscanner::QSPrintMode;
pub use crate::qscanner::QScanPingResult;
pub use crate::qscanner::QScanPingState;
pub use crate::qscanner::QScanResult;
pub use crate::qscanner::QScanTcpConnectResult;
pub use crate::qscanner::QScanTcpConnectState;
pub use crate::qscanner::QScanType;
pub use crate::qscanner::QScanner;

/// Module for asynchronous network ports scanning
pub mod qscanner;
