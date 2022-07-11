#![allow(unused)]

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_SHUTDOWN: usize = 8;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;


#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        // 应用程序访问操作系统提供的系统调用指令是ecall
        // 操作系统访问RustSBI提供的SBI调用指令也是ecall
        // 指令相同但特权级不同，应用程序处于用户级，操作系统处于内核特权级，RustSBI处于机器特权级
        core::arch::asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}


fn syscall(id: usize, args: [usize; 3]) -> isize {
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

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
