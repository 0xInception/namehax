use clap::Parser;
use process_list::for_each_module;
use process_list::for_each_process;
use std::ffi::c_void;
use std::{thread, time};
use winapi::shared::minwindef::{self};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Dump an offset from string
    #[clap(short, long)]
    dump: Option<String>,

    /// Specify custom offset. Default is for nameChangeFlag.
    #[clap(short, long, default_value_t = 20295280)]
    offset: i32,
}

fn main() {
    let args = Args::parse();
    println!("Searching for process...");
    let pid = get_pid_by_name("LeagueClient.exe".into());
    println!("Found process with pid {}", pid);
    let ten_millis = time::Duration::from_millis(250);
    thread::sleep(ten_millis);
    let modd = get_base_module_by_pid(pid, "LeagueClient.exe".into());
    println!("Found base module address {}", modd);
    println!("Opening handle...");
    let ten_millis = time::Duration::from_millis(250);
    thread::sleep(ten_millis);
    let handle = unsafe {
        winapi::um::processthreadsapi::OpenProcess(
            0x001F0FFF,
            winapi::shared::minwindef::FALSE,
            pid as minwindef::DWORD,
        )
    };
    match args.dump {
        Some(x) => {
            println!("Dumping offset. Please wait...");
            let addy = scan_for_string(handle, modd as i32, x);
            let offset = addy - modd as i32;
            println!("Offset: {}", offset);
            println!("You can use 'namehax.exe -o {}' on the next run!", offset);
        }
        None => {
            let address = args.offset + modd as i32;
            println!("{}", address);
            let mut buf = vec![0; 14];
            read_bytes(handle, address, &mut buf).unwrap();
            let x = String::from_utf8(buf);
            match x {
                Ok(f) => println!("Going to overwrite '{:?}'", f),
                Err(_) => println!("Invalid utf8"),
            }

            println!("Changing memory protection to readwrite!");
            virtual_protectex(handle, address, 0x40).unwrap();
            let mut buf: [u8; 2] = [111, 111];

            println!("Overwriting 2 bytes!");
            write_memory(handle, address, &mut buf).unwrap();
            println!("Changing memory protection to read!");
            virtual_protectex(handle, address, 0x02).unwrap();
        }
    }

    println!("Done!");
}
fn write_memory(handle: *mut c_void, address: i32, buf: &mut [u8]) -> Result<(), String> {
    unsafe {
        let result = winapi::um::memoryapi::WriteProcessMemory(
            handle,
            address as minwindef::LPVOID,
            buf.as_mut_ptr() as minwindef::LPVOID,
            buf.len() as winapi::shared::basetsd::SIZE_T,
            std::ptr::null_mut::<usize>(),
        );
        if result == 0 {
            return Err(format!("Could not write bytes! {}", address));
        }
        Ok(())
    }
}
fn virtual_protectex(handle: *mut c_void, address: i32, flag: u32) -> Result<(), String> {
    unsafe {
        let mut x: u32 = 0;
        let result = winapi::um::memoryapi::VirtualProtectEx(
            handle,
            address as minwindef::LPVOID,
            8,
            flag,
            &mut x,
        );
        if result == 0 {
            Err("Could not virtual protect.".into())
        } else {
            Ok(())
        }
    }
}
fn read_bytes(handle: *mut c_void, address: i32, buf: &mut Vec<u8>) -> Result<(), String> {
    unsafe {
        let result = winapi::um::memoryapi::ReadProcessMemory(
            handle,
            address as minwindef::LPVOID,
            buf.as_mut_ptr() as minwindef::LPVOID,
            buf.len() as winapi::shared::basetsd::SIZE_T,
            std::ptr::null_mut::<usize>(),
        );
        if result == 0 {
            return Err(format!("Could not read bytes! {}", address));
        }
        Ok(())
    }
}
fn scan_for_string(handle: *mut c_void, base_address: i32, str: String) -> i32 {
    let mut i = base_address;
    loop {
        let mut buf: Vec<u8> = vec![0; str.len()];
        read_bytes(handle, i, &mut buf).unwrap();
        if buf.eq(str.as_bytes()) {
            println!("Found string {} at address {}", &str, i);
            return i;
        }
        i += 1;
    }
}

fn get_base_module_by_pid(pid: u32, name: String) -> usize {
    let mut n: usize = 0;
    loop {
        if n != 0 {
            return n;
        }
        for_each_module(pid, |f, g| {
            let path = g.file_name().unwrap().to_str().unwrap();
            if path.contains(&name) {
                n = f.0;
                return;
            }
        })
        .unwrap();
    }
}
fn get_pid_by_name(name: String) -> u32 {
    let mut n: u32 = 0;
    loop {
        if n != 0 {
            return n;
        }
        for_each_process(|f, g| {
            let path = g.file_name().unwrap().to_str().unwrap();
            if path.contains(&name) {
                n = f;
            }
        })
        .unwrap();
        let ten_millis = time::Duration::from_millis(250);
        thread::sleep(ten_millis);
    }
}
