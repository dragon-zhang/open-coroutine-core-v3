pub mod adapter;

pub mod scheduler;

#[cfg(target_arch = "x86_64")]
const _JBLEN: usize = (9 * 2) + 3 + 16;

#[cfg(target_arch = "x86")]
const _JBLEN: usize = 18;

#[cfg(target_arch = "aarch64")]
const _JBLEN: usize = (14 + 8 + 2) * 2;

#[cfg(target_arch = "arm")]
const _JBLEN: usize = 10 + 16 + 2;

pub type JmpBuf = [libc::c_int; _JBLEN];

pub type SigJmpBuf = [libc::c_int; _JBLEN + 1];

extern "C" {

    pub fn setjmp(env: *mut JmpBuf) -> libc::c_int;

    pub fn longjmp(env: *mut JmpBuf, arg: libc::c_int);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unsafe fn func(mut buf: JmpBuf) {
            println!("call longjmp");
            longjmp(&mut buf, 1);
            println!("you will never see this because of longjmp");
        }
        unsafe {
            let mut buf: JmpBuf = std::mem::zeroed();
            if setjmp(&mut buf) != 0 {
                println!("setjmp back to main");
            } else {
                println!("setjmp first time through");
                func(buf);
            }
        }
    }
}
