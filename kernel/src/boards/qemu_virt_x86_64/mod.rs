// Copyright (c) 2025 vivo Mobile Communication Co., Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod init;
pub use init::*;
pub mod uart;
pub use uart::get_early_uart;
mod config;

pub(crate) fn get_cycles_to_duration(cycles: u64) -> core::time::Duration {
    // Using TSC (Time Stamp Counter) frequency approximation
    // This should be properly calibrated in a real implementation
    const TSC_FREQ_HZ: u64 = 2_000_000_000; // 2 GHz approximation
    core::time::Duration::from_nanos((cycles as f64 * (1_000_000_000f64 / TSC_FREQ_HZ as f64)) as u64)
}

pub(crate) fn get_cycles_to_ms(cycles: u64) -> u64 {
    const TSC_FREQ_HZ: u64 = 2_000_000_000; // 2 GHz approximation
    (cycles as f64 * (1_000f64 / TSC_FREQ_HZ as f64)) as u64
}