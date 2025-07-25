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

mod exception;
pub mod irq;
pub(crate) mod registers;

use crate::scheduler;
use core::{
    fmt,
    mem::offset_of,
    sync::{
        atomic,
        atomic::{AtomicU8, Ordering},
    },
};
use scheduler::ContextSwitchHookHolder;

pub(crate) const NR_SWITCH: usize = !0;

pub(crate) static READY_CORES: AtomicU8 = AtomicU8::new(0);

macro_rules! disable_interrupt {
    () => {
        "cli"
    };
}

macro_rules! enable_interrupt {
    () => {
        "sti"
    };
}

#[macro_export]
macro_rules! arch_bootstrap {
    ($stack_start:path, $stack_end:path, $cont: path) => {
        core::arch::naked_asm!(
            "
            mov rsp, {stack_end}
            jmp {cont}
            ",
            stack_end = sym $stack_end,
            cont = sym $cont,
        )
    };
}

#[macro_export]
macro_rules! x86_64_save_context_prologue {
    () => {
        "
        sub rsp, {stack_size}
        "
    };
}

#[macro_export]
macro_rules! x86_64_restore_context_epilogue {
    () => {
        "
        add rsp, {stack_size}
        "
    };
}

#[macro_export]
macro_rules! x86_64_save_context {
    () => {
        "
        mov [rsp + {rax}], rax
        mov [rsp + {rbx}], rbx
        mov [rsp + {rcx}], rcx
        mov [rsp + {rdx}], rdx
        mov [rsp + {rsi}], rsi
        mov [rsp + {rdi}], rdi
        mov [rsp + {rbp}], rbp
        mov [rsp + {r8}], r8
        mov [rsp + {r9}], r9
        mov [rsp + {r10}], r10
        mov [rsp + {r11}], r11
        mov [rsp + {r12}], r12
        mov [rsp + {r13}], r13
        mov [rsp + {r14}], r14
        mov [rsp + {r15}], r15
        pushfq
        pop rax
        mov [rsp + {rflags}], rax
        "
    };
}

#[macro_export]
macro_rules! x86_64_restore_context {
    () => {
        "
        mov rax, [rsp + {rflags}]
        push rax
        popfq
        mov rax, [rsp + {rax}]
        mov rbx, [rsp + {rbx}]
        mov rcx, [rsp + {rcx}]
        mov rdx, [rsp + {rdx}]
        mov rsi, [rsp + {rsi}]
        mov rdi, [rsp + {rdi}]
        mov rbp, [rsp + {rbp}]
        mov r8, [rsp + {r8}]
        mov r9, [rsp + {r9}]
        mov r10, [rsp + {r10}]
        mov r11, [rsp + {r11}]
        mov r12, [rsp + {r12}]
        mov r13, [rsp + {r13}]
        mov r14, [rsp + {r14}]
        mov r15, [rsp + {r15}]
        "
    };
}

#[derive(Default, Debug)]
#[repr(C, align(16))]
pub struct Context {
    pub rax: usize,
    pub rbx: usize,
    pub rcx: usize,
    pub rdx: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rbp: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub rip: usize,
    pub rflags: usize,
    pub rsp: usize,
    pub padding: usize,
}

impl Context {
    #[inline]
    pub(crate) fn init(&mut self) -> &mut Self {
        self.rflags = 0x202; // Enable interrupts by default
        self
    }

    #[inline(never)]
    pub(crate) fn set_return_address(&mut self, rip: usize) -> &mut Self {
        self.rip = rip;
        self
    }

    #[inline]
    pub(crate) fn set_arg(&mut self, index: usize, val: usize) -> &mut Self {
        match index {
            0 => self.rdi = val, // First argument in System V ABI
            1 => self.rsi = val,
            2 => self.rdx = val,
            3 => self.rcx = val,
            4 => self.r8 = val,
            5 => self.r9 = val,
            _ => {}
        }
        self
    }

    pub(crate) fn set_return_value(&mut self, val: usize) -> &mut Self {
        self.rax = val;
        self
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Context {{")?;
        write!(f, "rax: {:016x}, ", self.rax)?;
        write!(f, "rbx: {:016x}, ", self.rbx)?;
        write!(f, "rcx: {:016x}, ", self.rcx)?;
        write!(f, "rdx: {:016x}, ", self.rdx)?;
        write!(f, "rsi: {:016x}, ", self.rsi)?;
        write!(f, "rdi: {:016x}, ", self.rdi)?;
        write!(f, "rbp: {:016x}, ", self.rbp)?;
        write!(f, "r8: {:016x}, ", self.r8)?;
        write!(f, "r9: {:016x}, ", self.r9)?;
        write!(f, "r10: {:016x}, ", self.r10)?;
        write!(f, "r11: {:016x}, ", self.r11)?;
        write!(f, "r12: {:016x}, ", self.r12)?;
        write!(f, "r13: {:016x}, ", self.r13)?;
        write!(f, "r14: {:016x}, ", self.r14)?;
        write!(f, "r15: {:016x}, ", self.r15)?;
        write!(f, "rip: {:016x}, ", self.rip)?;
        write!(f, "rflags: {:016x}, ", self.rflags)?;
        write!(f, "rsp: {:016x}", self.rsp)?;
        write!(f, "}}")
    }
}

pub(crate) extern "C" fn is_in_interrupt() -> bool {
    false
}

#[inline(always)]
pub(crate) extern "C" fn switch_context(saved_sp_mut: *mut u8, to_sp: usize) {
    switch_context_with_hook(saved_sp_mut, to_sp, core::ptr::null_mut());
}

#[inline(always)]
#[allow(clippy::empty_loop)]
pub(crate) extern "C" fn restore_context_with_hook(
    to_sp: usize,
    hook: *mut ContextSwitchHookHolder,
) -> ! {
    switch_context_with_hook(core::ptr::null_mut(), to_sp, hook);
    loop {}
}

#[inline]
pub(crate) extern "C" fn switch_context_with_hook(
    saved_sp_mut: *mut u8,
    to_sp: usize,
    hook: *mut ContextSwitchHookHolder,
) {
    // For now, just a stub implementation
    // TODO: Implement proper context switching via syscall or interrupt
    // This would typically involve:
    // 1. Saving current context to saved_sp_mut
    // 2. Calling the hook if provided
    // 3. Restoring context from to_sp
    // 4. Jumping to the new context
}

#[naked]
pub(crate) extern "C" fn init(_: *mut u8, stack_end: *mut u8, cont: extern "C" fn()) {
    unsafe {
        core::arch::naked_asm!(
            "
            mov rsp, rsi
            jmp rdx
            "
        )
    }
}

#[no_mangle]
pub(crate) extern "C" fn start_schedule(cont: extern "C" fn() -> !) {
    let current = crate::scheduler::current_thread();
    current.lock().reset_saved_sp();
    let sp = current.saved_sp();
    drop(current);
    READY_CORES.fetch_add(1, Ordering::Relaxed);
    unsafe {
        core::arch::asm!(
            "mov rsp, {sp}",
            "jmp {cont}",
            sp = in(reg) sp,
            cont = in(reg) cont,
            options(noreturn),
        )
    }
}

#[inline]
pub extern "C" fn disable_local_irq() {
    unsafe { core::arch::asm!("cli", options(nostack, nomem)) }
}

#[inline]
pub extern "C" fn enable_local_irq() {
    unsafe { core::arch::asm!("sti", options(nostack, nomem)) }
}

#[inline]
pub extern "C" fn current_cpu_id() -> usize {
    // For now, just return 0 (single core)
    0
}

#[inline(always)]
pub(crate) extern "C" fn idle() {
    unsafe { core::arch::asm!("hlt", options(nostack)) };
}

#[inline]
pub(crate) extern "C" fn current_sp() -> usize {
    let x: usize;
    unsafe { core::arch::asm!("mov {}, rsp", out(reg) x, options(nostack, nomem)) };
    x
}

#[inline]
pub extern "C" fn disable_local_irq_save() -> usize {
    let old: usize;
    unsafe {
        core::arch::asm!(
            "pushfq",
            "pop {old}",
            concat!(disable_interrupt!()),
            old = out(reg) old,
            options(nostack)
        )
    }
    atomic::compiler_fence(Ordering::SeqCst);
    old
}

#[inline]
pub extern "C" fn enable_local_irq_restore(old: usize) {
    atomic::compiler_fence(Ordering::SeqCst);
    unsafe {
        core::arch::asm!(
            "push {old}",
            "popfq",
            old = in(reg) old,
            options(nostack)
        )
    }
}

#[inline]
pub extern "C" fn local_irq_enabled() -> bool {
    let x: usize;
    unsafe {
        core::arch::asm!(
            "pushfq",
            "pop {x}",
            x = out(reg) x,
            options(nostack)
        );
    };
    (x & (1 << 9)) != 0 // IF flag in RFLAGS
}

#[inline]
pub extern "C" fn pend_switch_context() {}

pub fn secondary_cpu_setup(_base: u32) {
    // TODO: Implement SMP support for x86_64
}