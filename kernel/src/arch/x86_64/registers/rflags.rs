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

//! RFLAGS register definitions  

use tock_registers::{register_bitfields, registers::ReadWrite};

register_bitfields! [usize,
    RFLAGS [
        CF OFFSET(0) NUMBITS(1) [],   // Carry Flag
        PF OFFSET(2) NUMBITS(1) [],   // Parity Flag  
        AF OFFSET(4) NUMBITS(1) [],   // Auxiliary Carry Flag
        ZF OFFSET(6) NUMBITS(1) [],   // Zero Flag
        SF OFFSET(7) NUMBITS(1) [],   // Sign Flag
        TF OFFSET(8) NUMBITS(1) [],   // Trap Flag
        IF OFFSET(9) NUMBITS(1) [],   // Interrupt Enable Flag
        DF OFFSET(10) NUMBITS(1) [],  // Direction Flag
        OF OFFSET(11) NUMBITS(1) [],  // Overflow Flag
        IOPL OFFSET(12) NUMBITS(2) [], // I/O Privilege Level
        NT OFFSET(14) NUMBITS(1) [],  // Nested Task
        RF OFFSET(16) NUMBITS(1) [],  // Resume Flag
        VM OFFSET(17) NUMBITS(1) [],  // Virtual 8086 Mode
        AC OFFSET(18) NUMBITS(1) [],  // Alignment Check
        VIF OFFSET(19) NUMBITS(1) [], // Virtual Interrupt Flag
        VIP OFFSET(20) NUMBITS(1) [], // Virtual Interrupt Pending
        ID OFFSET(21) NUMBITS(1) []   // ID Flag
    ]
];

pub static RFLAGS: ReadWrite<usize, RFLAGS::Register> = ReadWrite::new(0);