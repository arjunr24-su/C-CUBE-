use std::ffi::OsStr;
use std::io::{self, Write};
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use windows::Win32::Foundation::{CloseHandle, BOOL, GetLastError};
use windows::Win32::System::Threading::{
    CreateProcessW, OpenProcess, ResumeThread, SuspendThread, TerminateProcess, PROCESS_INFORMATION,
    STARTUPINFOW, PROCESS_ALL_ACCESS, PROCESS_CREATION_FLAGS,
};
use windows::core::{PCWSTR, PWSTR};

// Function to convert a string to PCWSTR (Windows String format)
fn to_pcwstr(s: &str) -> PCWSTR {
    let wide: Vec<u16> = OsStr::new(s).encode_wide().chain(Some(0)).collect();
    PCWSTR::from_raw(wide.as_ptr())
}

// Function to create a process
fn create_process(app_name: &str) -> Option<PROCESS_INFORMATION> {
    let mut si = STARTUPINFOW::default();
    let mut pi = PROCESS_INFORMATION::default();
    let app_path = to_pcwstr(app_name);

    let result: BOOL = unsafe {
        CreateProcessW(
            app_path,
            PWSTR(app_path.0 as *mut u16), // Use app_path for lpCommandLine
            None,
            None,
            false,
            PROCESS_CREATION_FLAGS(0),
            None,
            None,
            &mut si,
            &mut pi,
        )
    };

    if result.as_bool() {
        println!("Process created successfully. PID: {}", pi.dwProcessId);
        Some(pi)
    } else {
        let error_code = unsafe { GetLastError().0 };
        println!("Failed to create process. Error code: {}", error_code);
        None
    }
}

// Function to terminate a process by PID
fn terminate_process(pid: u32) -> bool {
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid).expect("Failed to open process") };
    if handle.is_invalid() {
        println!("Invalid process handle.");
        return false;
    }

    let result = unsafe { TerminateProcess(handle, 1) };
    unsafe { CloseHandle(handle) };
    result.as_bool()
}

// Function to suspend a process by PID
fn suspend_process(pid: u32) -> bool {
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid).expect("Failed to open process") };
    if handle.is_invalid() {
        println!("Invalid process handle.");
        return false;
    }

    let result = unsafe { SuspendThread(handle) };
    unsafe { CloseHandle(handle) };
    result != u32::MAX
}

// Function to resume a process by PID
fn resume_process(pid: u32) -> bool {
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid).expect("Failed to open process") };
    if handle.is_invalid() {
        println!("Invalid process handle.");
        return false;
    }

    let result = unsafe { ResumeThread(handle) };
    unsafe { CloseHandle(handle) };
    result != u32::MAX
}

fn main() {
    loop {
        print!("Enter command (create <app>, terminate <pid>, suspend <pid>, resume <pid>, exit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "create" => {
                if parts.len() < 2 {
                    println!("Usage: create <app_name>");
                } else {
                    create_process(parts[1]);
                }
            }
            "terminate" => {
                if parts.len() < 2 {
                    println!("Usage: terminate <pid>");
                } else if let Ok(pid) = parts[1].parse::<u32>() {
                    if terminate_process(pid) {
                        println!("Process terminated.");
                    } else {
                        println!("Failed to terminate process.");
                    }
                } else {
                    println!("Invalid PID.");
                }
            }
            "suspend" => {
                if parts.len() < 2 {
                    println!("Usage: suspend <pid>");
                } else if let Ok(pid) = parts[1].parse::<u32>() {
                    if suspend_process(pid) {
                        println!("Process suspended.");
                    } else {
                        println!("Failed to suspend process.");
                    }
                } else {
                    println!("Invalid PID.");
                }
            }
            "resume" => {
                if parts.len() < 2 {
                    println!("Usage: resume <pid>");
                } else if let Ok(pid) = parts[1].parse::<u32>() {
                    if resume_process(pid) {
                        println!("Process resumed.");
                    } else {
                        println!("Failed to resume process.");
                    }
                } else {
                    println!("Invalid PID.");
                }
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Unknown command.");
            }
        }
    }
}
