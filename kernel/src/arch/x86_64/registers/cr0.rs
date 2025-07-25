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

//! CR0 register definitions

use tock_registers::{register_bitfields, registers::ReadWrite};

register_bitfields! [usize,
    CR0 [
        PE OFFSET(0) NUMBITS(1) [],  // Protection Enable
        MP OFFSET(1) NUMBITS(1) [],  // Monitor Coprocessor
        EM OFFSET(2) NUMBITS(1) [],  // x87 FPU Emulation
        TS OFFSET(3) NUMBITS(1) [],  // Task Switched
        ET OFFSET(4) NUMBITS(1) [],  // Extension Type
        NE OFFSET(5) NUMBITS(1) [],  // Numeric Error
        WP OFFSET(16) NUMBITS(1) [], // Write Protect
        AM OFFSET(18) NUMBITS(1) [], // Alignment Mask
        NW OFFSET(29) NUMBITS(1) [], // Not Write-through
        CD OFFSET(30) NUMBITS(1) [], // Cache Disable
        PG OFFSET(31) NUMBITS(1) []  // Paging
    ]
];

pub static CR0: ReadWrite<usize, CR0::Register> = ReadWrite::new(0);