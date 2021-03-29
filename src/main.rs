
use std::env;
use std::fs;
extern crate xmas_elf;
use xmas_elf::ElfFile;
use xmas_elf::header::Class;
use xmas_elf::dynamic;
use xmas_elf::sections;
use xmas_elf::symbol_table::Entry;

fn check_sec_64(elf : &ElfFile){
    let mut canary = false;
    let mut pie = false;
    let mut nx = true;
    for section in elf.section_iter(){
        //check for canary, get every data section
        canary = match section.get_data(&elf){
            //match on DynSymbol
            Ok(sections::SectionData::DynSymbolTable64(dynsyn_tab))=>{
                let mut iter = dynsyn_tab.iter();
                iter.any(|dynsyn|{
                    //check every section with stack chk fail
                    dynsyn.get_name(&elf)==Ok("__stack_chk_fail")
                })
            },
            _ => canary,
        };
        //check for PIE, check every section ? 
        if let Ok(sections::SectionData::Dynamic64(dynamic_tab))
                = section.get_data(&elf){
                for dynamic in dynamic_tab{
                    if let Ok(dynamic::Tag::Flags1) = dynamic.get_tag(){
                        pie = dynamic::FLAG_1_PIE != 0x0;
                    }
                }
            }
        }
    //to test on a non-nx prog
    for program in elf.program_iter(){
        if program.flags().is_execute() && program.flags().is_write(){
            nx = false
        }
    }
    println!("Canary {}",canary);
    println!("NX {}",nx);
    println!("Pie {}",pie);
}

fn main() {
    let arg:Vec<String> = env::args().collect();
    if arg.len()< 2{
        panic!("Need a binary to parse!");
    }
    let file = &arg[1];
    let content = fs::read(file).expect("Error reading file!");
    let elf_file = ElfFile::new(&content).expect("Failed to parse elf");
    
    match elf_file.header.pt1.class.as_class(){
        Class::ThirtyTwo => println!("not done"),
        Class::SixtyFour => check_sec_64(&elf_file),
        _ => println!("It something else"),
    };

}
