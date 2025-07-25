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

//! CR3 register definitions

use tock_registers::{register_bitfields, registers::ReadWrite};

register_bitfields! [usize,
    CR3 [
        PWT OFFSET(3) NUMBITS(1) [], // Page-level Write-Through
        PCD OFFSET(4) NUMBITS(1) [], // Page-level Cache Disable
        PDB OFFSET(12) NUMBITS(40) [] // Page Directory Base
    ]
];

pub static CR3: ReadWrite<usize, CR3::Register> = ReadWrite::new(0);