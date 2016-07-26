extern crate rust_elf32;
use rust_elf32::elf::{SectionHeader, ElfHeader};

const MAGIC_COPIED: u32 = 0x0DD001C0;

fn read_elf_header(ptr:usize){
    let mut ptr = ptr as *mut ElfHeader;
    unsafe{println!("{:?}", *ptr)};
}

fn read_all_sections(head_wrapper: &elf::ElfHeadWrapper) {
    unsafe {
    println!("{:?}", head_wrapper.header);

    for entry in head_wrapper.get_sections_headers() {
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

fn get_size_of_metadata(head_wrapper: &elf::ElfHeadWrapper) -> usize {
    unsafe {
    let mut total_size = mem::size_of::<elf::ElfHeader>();

    total_size += mem::size_of::<elf::SectionHeader>() * head_wrapper.get_sections_headers().len();
    total_size += head_wrapper.get_section(elf::Section::STRTAB).size() as usize;
    total_size
    }
}

fn check_for_our_magic(outfile: &mut File) -> bool {

    let mut array = [0;4];
    let ref mut magic_end = array;

    outfile.read(magic_end).unwrap();

    unsafe{
        let magic_be: [u8;4] = mem::transmute(MAGIC_COPIED);

        assert!(magic_end != &magic_be);
    }

    false
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
use std::io::{self, Write, Seek, Read};
use std::fs::File;
use std::env;
use std::mem;
use rust_elf32::elf;

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

    // Check for the magic ending to see if we have already copied the 
    // elf headers to this file.
    let mut outfile = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open(&outpath)
                            .unwrap();

    // If there is an error seeking, assume that the file is empty.
    match outfile.seek(io::SeekFrom::End( -(mem::size_of::<u32>() as i64))) {
        Ok(_) => {check_for_our_magic(&mut outfile);},
        Err(e) => println!("{:?}", e)
    }


    let metadata_size = unsafe {

        let mut elf_header = elf::ElfHeadWrapper::new(&mut*(elf_ptr as *mut ElfHeader));

        read_all_sections(&elf_header);
        
        assert!(elf_header.test_valid());

        let section_headers = elf_header.get_sections_headers();

        let strtab = elf_header.get_str_table(section_headers);

        for c in strtab {
            if *c != 0 {
                print!("{:}", *c as char);
            }
            else {
                println!("");
            }

        }

        // Get the size of the elf info
        get_size_of_metadata(&elf_header)
    };

    // Scope the out_mmap
    {
        // Allocate space for metadata in the file first
        outfile.seek(io::SeekFrom::End(metadata_size as i64)).unwrap();

        //TODO: Check the end of the file for a magic number so we don't rewrite.
        
        outfile.write(&[0]).unwrap();

        let mut out_mmap = memmap::Mmap::open(&outfile, memmap::Protection::ReadWrite).unwrap();

        let mut out_mmap_addr = out_mmap.mut_ptr() as usize;
        assert!(out_mmap_addr != 0);

        unsafe {
            let mut elf_header = elf::ElfHeadWrapper::new(&mut*(elf_ptr as *mut ElfHeader));

            elf_header.copy(out_mmap_addr);
        }
    }

    outfile.seek(io::SeekFrom::End(0)).unwrap();

    unsafe {
        let magic: [u8; 4] = mem::transmute(MAGIC_COPIED);
        outfile.write(&magic).unwrap();
    }
}
