use std::arch::asm;
use std::ffi::{
    c_void,
    OsStr,
    CStr,
};
use std::os::unix::ffi::OsStrExt;
use std::os::raw::{
    c_int,
    c_char,
};

fn main() { unsafe {
    let lib_ptr = load(OsStr::new("/lib64/libc.so.6")).unwrap();
    let sym_ptr = get_sym(
        lib_ptr,
        CStr::from_ptr("printf\0".as_ptr() as *const c_char),
    ).unwrap();

    let args = vec![
"
%s
%s
%s
%s
%s
%s
%s
%s
\0".as_ptr() as usize,
        "1\0".as_ptr() as usize,
        "2\0".as_ptr() as usize,
        "3\0".as_ptr() as usize,
        "4\0".as_ptr() as usize,
        "5\0".as_ptr() as usize,
        "6\0".as_ptr() as usize,
        "7\0".as_ptr() as usize,
        "8\0".as_ptr() as usize,
    ];

    let ret = call(sym_ptr, args);
    println!("ret: {}", ret);
}}

unsafe fn call(fn_ptr: *mut (), args: Vec<usize>) -> usize {
    let exit = std::process::exit as *const ();
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
            let arr = args.into_boxed_slice();

            asm!(
                "push rbp",
                "mov rbp, rsp",

                "mov rax, {0}",
                "2:",
                "cmp rax, 5",
                "jle 2f",
                "mov r15, rax",
                "shl r15, 3",
                "add r15, {1}",

                "push [r15]",
                "sub rax, 1",
                "jmp 2b",

                "2:",
                "xor rax, rax",
                "call {2}",

                "mov rsp, rbp",
                "pop rbp",

                in(reg) len - 1,
                in(reg) arr.as_ptr(),
                in(reg) fn_ptr,
                in("rdi") arr[0],
                in("rsi") arr[1],
                in("rdx") arr[2],
                in("rcx") arr[3],
                in("r8") arr[4],
                in("r9") arr[5],
                in("r14") exit,
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
