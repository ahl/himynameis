#![feature(asm)]

use std::fs::File;

use addr2line::{
    object::{Object, ObjectSection, ObjectSegment},
    Context,
};

fn main() {
    slim();
}
fn slim() {
    shady();
}
fn shady() {
    let rip = unsafe {
        let rip: u64;
        asm!("lea {0}, [rip+0]", out(reg) rip);
        rip
    };
    println!(
        "Hello, world! {} {:x}",
        std::env::current_exe().unwrap().to_string_lossy(),
        rip
    );


    let path = std::env::current_exe().unwrap();
    let file = File::open(path).unwrap();
    let map = unsafe { memmap::Mmap::map(&file).unwrap() };
    let object = &addr2line::object::File::parse(&*map).unwrap();
    let symbols = object.symbol_map();
    let context = Context::new(object).unwrap();

    for seg in object.segments() {
        println!("seg {:?} {:x?}", seg.name(), seg.address());
    }
    for sec in object.sections() {
        println!("sec {:?} {:x?}", sec.name(), sec.address());
    }

    //let rip = rip - (v as u64) + text_sec.address();
    println!("{} {:x}", rip, rip);

    let sym = symbols.get(rip);

    println!("{:?}", sym);

    let mut frames = context.find_frames(rip).unwrap();

    while let Some(frame) = frames.next().unwrap() {

        println!("{:?}", frame.function.unwrap().raw_name());
    }


    let xx = context.find_location(rip).unwrap().unwrap();

    println!("{:?}", xx.line);
}
