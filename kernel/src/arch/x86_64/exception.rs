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

//! x86_64 exception handling

// x86_64 exception vectors
pub const DIVIDE_ERROR: usize = 0;
pub const DEBUG: usize = 1;
pub const NMI: usize = 2;
pub const BREAKPOINT: usize = 3;
pub const OVERFLOW: usize = 4;
pub const BOUND_RANGE_EXCEEDED: usize = 5;
pub const INVALID_OPCODE: usize = 6;
pub const DEVICE_NOT_AVAILABLE: usize = 7;
pub const DOUBLE_FAULT: usize = 8;
pub const INVALID_TSS: usize = 10;
pub const SEGMENT_NOT_PRESENT: usize = 11;
pub const STACK_SEGMENT_FAULT: usize = 12;
pub const GENERAL_PROTECTION_FAULT: usize = 13;
pub const PAGE_FAULT: usize = 14;
pub const X87_FP_EXCEPTION: usize = 16;
pub const ALIGNMENT_CHECK: usize = 17;
pub const MACHINE_CHECK: usize = 18;
pub const SIMD_FP_EXCEPTION: usize = 19;
pub const VIRTUALIZATION: usize = 20;
pub const CONTROL_PROTECTION: usize = 21;

pub fn init_exceptions() {
    // TODO: Initialize exception handlers
}

pub fn handle_exception(vector: usize, error_code: Option<u64>) {
    // TODO: Implement exception handling
    match vector {
        DIVIDE_ERROR => panic!("Divide by zero error"),
        DEBUG => panic!("Debug exception"),
        BREAKPOINT => panic!("Breakpoint exception"),
        GENERAL_PROTECTION_FAULT => panic!("General protection fault, error code: {:?}", error_code),
        PAGE_FAULT => panic!("Page fault, error code: {:?}", error_code),
        _ => panic!("Unhandled exception: {}, error code: {:?}", vector, error_code),
    }
}