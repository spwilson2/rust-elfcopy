#![allow(unused_variables, dead_code)]
extern crate rust_elf32;
extern crate memmap;

use std::process::exit;
use std::fs::{OpenOptions, File};
use std::io::{self, Write, Seek, Read};
use std::env;
use std::mem;
use rust_elf32::elf::{self, ElfHeader};
use memmap::Mmap;

/// Magic number used to indicate that we have already written to the output file.
const MAGIC_COPIED: u32 = 0x0DD001C0;

/// Print all the section headers as well as the elf header.
fn read_all_sections(head_wrapper: &elf::ElfHeadWrapper) {

    // TODO: Need to make a format/string function for the headers since we don't want
    // to rely on debug (Viewing hex in decimal is annoying).
    unsafe {
        println!("{:?}", head_wrapper.header);

        for entry in head_wrapper.get_sections_headers() {
            println!("{:?}", entry);
        }
    }
}

///
/// Map the input file into memory as read only.
/// Return a (usize, memmap::Mmap) which are the address the memmap begins at and the
/// memory map object respectively. The Mmap must remain in scope as long as you expect
/// to use the pointer to it, otherwise it will be unmapped.
///
fn map_input_file(path: &String) -> (usize, Mmap) {

    let mut mmap = match memmap::Mmap::open_path(&path, memmap::Protection::Read) {
        Ok(mmap) => mmap,
        Err(_) => {
            println!("Unable to mmap {} for reading, does it exists?", path);
            exit(1);
        }
    };

    let ptr = mmap.mut_ptr();
    if ptr as usize == 0 {
        panic!("Could not access data from {}", path)
    }

    println!("{:x}", ptr as usize);

    return (ptr as usize, mmap) // Need to keep the mmap so it doesn't get unmapped.
}

///
/// Return the total size of the Elf Header, Section Headers, and strtab.
///
fn get_size_of_metadata(head_wrapper: &elf::ElfHeadWrapper) -> usize {
    unsafe {
    let mut total_size = mem::size_of::<elf::ElfHeader>();

    total_size += mem::size_of::<elf::SectionHeader>() * head_wrapper.get_sections_headers().len();
    total_size += head_wrapper.get_section(elf::Section::STRTAB).size() as usize;
    total_size
    }
}

///
/// Check the given File for our magic number, return true if
/// it exists, false if it doesn't.
///
fn check_for_our_magic(outfile: &mut File) -> bool {

    let mut array = [0;4];
    let ref mut magic_end = array;

    outfile.read(magic_end).unwrap();

    unsafe{
        let magic_be: [u8;4] = mem::transmute(MAGIC_COPIED.to_le());

        magic_end == &magic_be
    }
}

///
/// Take in an infile, and an outfile and copy elf information into the end of the outfile.
/// This is meant to be able to use the binary as if it were multibooted and handed an elf
/// header.
///
fn main() {

    //
    // Panic if fail to find arg0.
    //
    let arg0 = env::args().nth(0).unwrap();

    let usage = format!("usage: \"{:} <in_elf_file> <out_bin_file>\"", arg0);

    let inpath  = match env::args().nth(1) {
        Some(arg) => arg,
        None  => {
            println!("{}", usage);
            return
        }
    };

    let outpath = match env::args().nth(2) {
        Some(arg) => arg,
        None  => {
            println!("{}", usage);
            return
        }
    };

    // Hold on to the referece to the in_mmap so it doesn't become unmapped.
    let (elf_ptr, inmmap) = map_input_file(&inpath);

    let mut outfile = match OpenOptions::new()
                            .read(true)
                            .write(true)
                            .open(&outpath) {
            Ok(file) => file,
            Err(_) => {
                println!("Unable to open {} for rw, does it exists, and is it writable?", outpath);
                return
            }
        };

    // If there is an error seeking, assume that the file is empty.
    match outfile.seek(io::SeekFrom::End( -(mem::size_of::<u32>() as i64))) {
        Ok(_) => 
                if check_for_our_magic(&mut outfile) {
                    println!("This file already has been elfcopied!");
                    return
                },
        Err(_) => (),
    }


    let metadata_size = unsafe {

        let elf_header = elf::ElfHeadWrapper::new(&mut*(elf_ptr as *mut ElfHeader));
        
        if !elf_header.test_valid() {
            println!("The given infile is not an .elf-32 format.");
            return
        }

        // Get the size of the elf info
        get_size_of_metadata(&elf_header)
    };

    // Scope the out_mmap
    {
        // The size of the file is needed so we can get the offset into the memory map
        // to place the elf header.
        let metadata = outfile.metadata().unwrap();
        let elf_offset = metadata.len() as usize;
        
        
        // Allocate space for metadata in the file first
        outfile.seek(io::SeekFrom::End(metadata_size as i64)).unwrap();

        //TODO: Check the end of the file for a magic number so we don't rewrite.
        
        outfile.write(&[0]).unwrap();

        let mut out_mmap = memmap::Mmap::open(&outfile, memmap::Protection::ReadWrite).unwrap();

        let out_mmap_addr = out_mmap.mut_ptr() as usize;
        assert!(out_mmap_addr != 0);

        unsafe {
            let elf_header = elf::ElfHeadWrapper::new(&mut*(elf_ptr as *mut ElfHeader));

            elf_header.copy(out_mmap_addr + elf_offset);
        }
    }

    outfile.seek(io::SeekFrom::End(0)).unwrap();

    // Tag the out file with our magic number.
    unsafe {
        let magic: [u8; 4] = mem::transmute(MAGIC_COPIED.to_le());
        outfile.write(&magic).unwrap();
    }
}
