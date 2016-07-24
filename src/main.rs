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

use rust_elf32::elf::{SectionHeader, ElfHeader};

/// Map the given file names in memory
//fn open_files(infile: &str, outfile: &str) -> Result<(Mmap,Mmap), memmap::Error> {
//}

fn read_elf_header(ptr:usize){
    let mut ptr = ptr as *mut ElfHeader;
    unsafe{println!("{:?}", *ptr)};
}

fn read_all_sections(ptr:usize) {
    unsafe {
    let mut header = &mut *(ptr as *mut ElfHeader);//ElfHeader::get_header(ptr as *mut ElfHeader);
    println!("{:?}", header as *mut ElfHeader);
    println!("{:?}", header);

    for entry in (*header).get_sections_headers() {
        println!("{:?}", entry);
    }
    }
}

fn map_file(path: &String, write: bool) -> (usize, Mmap) {
    let mut mmap = match write {
        true => memmap::Mmap::open_path(&path, memmap::Protection::ReadWrite).unwrap(),
        false => memmap::Mmap::open_path(&path, memmap::Protection::Read).unwrap(),
    };

    let mut ptr = mmap.mut_ptr();
    if ptr as usize == 0 {
        panic!("Could not access data from {}", path)
    }

    println!("{:x}", ptr as usize);

    return (ptr as usize, mmap) // Need to keep the mmap so it doesn't get unmapped.
}


//fn read_elf_section_header() -> {
//}

extern crate libc;
extern crate memmap;
use memmap::Mmap;
use std::os;
use std::ptr;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write, Seek};
use std::fs::File;
use std::env;

static usage: &'static str = "usage: {arg0} <in_elf_file> <out_bin_file>";

fn main() {

    let inpath  = env::args().nth(1).expect(usage);
    let outpath = env::args().nth(2).expect(usage);
    //    let mut f = OpenOptions::new()
    //                        .read(true)
    //                        .write(true)
    //                        .create(true)
    //                        .open(&path).unwrap();
    //
    // Allocate space in the file first
    //f.seek(io::SeekFrom::Start(size)).unwrap();
    //f.write(&[0]).unwrap();
    //f.seek(io::SeekFrom::Start(0)).unwrap();

    //let mut mmap = memmap::Mmap::open(&f, memmap::Protection::ReadWrite).unwrap();
    
    //TODO: Open the in file, read all the sections and headers, find out the length needed to
    //write them to memory. Then extend the out file by that much and fill.
    
    let (mut elf_ptr, mut inmmap) = map_file(&inpath, false);

    read_all_sections(elf_ptr);
    
    unsafe{assert!((&mut*(elf_ptr as *mut ElfHeader)).test_valid())}

    //   let src = "Hello!";
    //   let src_data = src.as_bytes();


    //   unsafe {
    //       ptr::copy(src_data.as_ptr(), &mut *(ptr as *mut _), src_data.len());
    //   }
    //read_elf_header(ptr as usize);
    //let ptr = (ptr as usize + 89336) as *mut usize;
    //let ref mut src = 1;
    //    unsafe{ptr::copy(src, &mut *(ptr as *mut _), 1)};
    //read_all_sections(ptr as usize);
}
