extern crate rust_elf32;
//extern crate libc;
//extern crate memmap;

//use std::io::{self, Read, Write, Error, Seek};
//use std::fs::File;

//fn main() {
//    match write_file() {
//        Ok(_) => {},
//        Err(e) => panic!("{}", e),
//    }
//}
//
//fn write_file() -> Result<(),std::io::Error> {
//    let mut userin = String::new();
//
//    io::stdout().write(b"Enter a file to dump elf of: ");
//    io::stdout().flush();
//
//    io::stdin().read_line(&mut userin);
//    let mut userin = userin.trim();
//    io::stdout().write(userin.as_bytes());
//
//    let mut file = try!(OpenOptions::new()
//                        .read(true)
//                        //.write(true)
//                        //.create(true)
//                        .open(userin));
//
//    assert_eq!(try!(file.write(b"This is text.\n")), 14);
//    Ok(())
//}

use rust_elf32::elf::ElfHeader;

/// Map the given file names in memory
//fn open_files(infile: &str, outfile: &str) -> Result<(Mmap,Mmap), memmap::Error> {
//}
//
fn read_elf_header(ptr:usize){
    let mut ptr = ptr as *mut ElfHeader;
    unsafe{println!("{:?}", *ptr)};
}

//fn read_elf_section_header() -> {
//}

extern crate libc;
extern crate memmap;
use std::os;
use std::ptr;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write, Seek};
use std::fs::File;

fn main() {
    let size = 1024;

    let path = "kernel.elf";
    let mut f = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&path).unwrap();

    // Allocate space in the file first
    //f.seek(io::SeekFrom::Start(size)).unwrap();
    //f.write(&[0]).unwrap();
    //f.seek(io::SeekFrom::Start(0)).unwrap();

    let mut mmap = memmap::Mmap::open(&f, memmap::Protection::ReadWrite).unwrap();

    let mut ptr = mmap.mut_ptr();

    if ptr as usize == 0 {
        panic!("Could not access data from memory mapped file")
    }

 //   let src = "Hello!";
 //   let src_data = src.as_bytes();

 //   unsafe {
 //       ptr::copy(src_data.as_ptr(), &mut *(ptr as *mut _), src_data.len());
 //   }
 read_elf_header(ptr as usize);
}
