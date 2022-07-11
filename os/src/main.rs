#![no_std]
#![no_main]
#![feature(panic_info_message)]


use log::*;


#[macro_use]
mod console;
mod lang_items;
mod logging;

// 使用global_asm宏将entry.asm嵌入到代码中
core::arch::global_asm!(include_str!("entry.asm"));


const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;
const SBI_SHUTDOWN: usize = 8;


#[inline(always)]
fn syscall(id: usize, args: [usize; 3]) -> usize {
    let mut ret;
    unsafe {
        // 应用程序访问操作系统提供的系统调用指令是ecall
        // 操作系统访问RustSBI提供的SBI调用指令也是ecall
        // 指令相同但特权级不同，应用程序处于用户级，操作系统处于内核特权级，RustSBI处于机器特权级
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> usize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> usize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn console_putchar(c: usize) {
    syscall(SBI_CONSOLE_PUTCHAR, [c, 0, 0]);
}

pub fn console_getchar() -> usize {
    syscall(SBI_CONSOLE_GETCHAR, [0, 0, 0])
}

pub fn shutdown() -> ! {
    syscall(SBI_SHUTDOWN, [0, 0, 0]);
    panic!("It should shutdown!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

// #[no_mangle]
// extern "C" fn _start() {
//     // loop {};
//     println!("Hello World");
//     sys_exit(9);
//     shutdown();
// }

// 标记为no_mangle避免编译器对名字混淆，否则entry.asm在链接时找不到rust_main
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("Hello, world!");
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
    // shutdown();
}