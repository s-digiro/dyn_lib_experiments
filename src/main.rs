use std::arch::asm;
use std::ffi::{
    c_void,
    c_int,
    c_char,
    OsStr,
    CStr,
};
use std::os::unix::ffi::OsStrExt;

fn main() { unsafe {
    let lib_ptr = load(OsStr::new("/lib/libc.so.6")).unwrap();
    let sym_ptr = get_sym(
        lib_ptr,
        CStr::from_ptr("printf\0".as_ptr() as *const c_char),
    ).unwrap();

    let args = vec![
        "%s\n%s\n%s\n%s\n%s\n%d\n\0".as_ptr() as usize,
        "Hello world!\0".as_ptr() as usize,
        "tits\0".as_ptr() as usize,
        "1\0".as_ptr() as usize,
        "2\0".as_ptr() as usize,
        //"3\0".as_ptr() as usize,
        //"4\0".as_ptr() as usize,
        "5\0".as_ptr() as usize,
        6
    ];

    call(sym_ptr, args);
}}

unsafe fn call(fn_ptr: *mut (), args: Vec<usize>) -> usize {
    let result;

    match args.len() {
        0 => asm!(
            "call {}",
            in(reg) fn_ptr,
            out("rax") result,
            clobber_abi("C"),
        ),
        1 => asm!(
            "call {}",
            in(reg) fn_ptr,
            in("rdi") args[0],
            out("rax") result,
            clobber_abi("C"),
        ),
        2 => asm!(
            "call {}",
            in(reg) fn_ptr,
            in("rdi") args[0],
            in("rsi") args[1],
            out("rax") result,
            clobber_abi("C"),
        ),
        3 => asm!(
            "call {}",
            in(reg) fn_ptr,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            out("rax") result,
            clobber_abi("C"),
        ),
        4 => asm!(
            "call {}",
            in(reg) fn_ptr,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            in("rcx") args[3],
            out("rax") result,
            clobber_abi("C"),
        ),
        5 => asm!(
            "call {}",
            in(reg) fn_ptr,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            in("rcx") args[3],
            in("r8") args[4],
            out("rax") result,
            clobber_abi("C"),
        ),
        6 => asm!(
            "call {}",
            in(reg) fn_ptr,
            in("rdi") args[0],
            in("rsi") args[1],
            in("rdx") args[2],
            in("rcx") args[3],
            in("r8") args[4],
            in("r9") args[5],
            out("rax") result,
            clobber_abi("C"),
        ),
        len => {
            asm!(
                "2:",
                "cmp {0}, 5",
                "jle 2f",
                "add {1}, {0}",
                "push [{1}]",
                "sub {1}, {0}",
                "add {0}, 1",
                "jmp 2b",
                "2:",
                in(reg) len,
                in(reg) args.as_ptr(),
            );

            asm!(
                "call {}",
                in(reg) fn_ptr,
                in("rdi") args[0],
                in("rsi") args[1],
                in("rdx") args[2],
                in("rcx") args[3],
                in("r8") args[4],
                in("r9") args[5],
                out("rax") result,
                clobber_abi("C"),
            );
        },
    }

    result
}

#[link(name = "c")]
extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

const RTLD_LOCAL: c_int = 0;
const RTLD_LAZY: c_int = 1;

pub unsafe fn load(path: &OsStr) -> Result<*mut c_void, String> {
    let mut v: Vec<u8> = Vec::new();

    let cstr = if path.len() > 0 && path.as_bytes()[path.len() - 1] == 0 {
        CStr::from_bytes_with_nul_unchecked(path.as_bytes())
    } else {
        v.extend_from_slice(path.as_bytes());
        v.push(0);
        CStr::from_bytes_with_nul_unchecked(v.as_slice())
    };

    let handle = dlopen(cstr.as_ptr(), RTLD_LOCAL | RTLD_LAZY);
    if handle.is_null() {
        Err("Could not load lib".into())
    } else {
        Ok(handle)
    }
}

pub unsafe fn get_sym(lib: *mut c_void, name: &CStr) -> Result<*mut (), String> {
    let sym = dlsym(lib, name.as_ptr());
    if sym.is_null() {
        Err("Could not find symbol".into())
    } else {
        Ok(sym as *mut ())
    }
}
