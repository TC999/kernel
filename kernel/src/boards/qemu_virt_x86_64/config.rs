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

use crate::arch::irq::IrqNumber;

// QEMU x86_64 "virt" machine configuration
pub const UART0_BASE: u64 = 0x3f8; // Standard COM1 port
pub const HEAP_SIZE: u64 = 16 * 1024 * 1024;
pub const DRAM_BASE: u64 = 0x100000; // 1MB - above legacy regions
pub const UART0_IRQNUM: IrqNumber = IrqNumber::new(4); // COM1 IRQ