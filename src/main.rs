use std::fs::File;
use std::io::{Read, Cursor};
use mach_object::{OFile, CPU_TYPE_X86_64, MachCommand, LoadCommand, SymbolIter, Symbol};

fn main() {
    let mut kern = File::open("/System/Library/Kernels/kernel").expect("Unable to open.");
    let mut buf = Vec::new();
    let size = kern.read_to_end(&mut buf).unwrap();
    let mut cur = Cursor::new(&buf[..size]);
    let mut symcur = Cursor::new(&buf[..size]);
    if let OFile::MachFile { ref header, ref commands } = OFile::parse(&mut cur).unwrap() {
        assert_eq!(header.cputype, CPU_TYPE_X86_64);
        assert_eq!(header.ncmds as usize, commands.len());
        println!("cpu type: {}", header.cputype);
        println!("# cmds: {}", header.ncmds);
        println!("magic: 0x{:x}", header.magic);
        for &MachCommand(ref cmd, cmdsize) in commands {
            if let &LoadCommand::Segment64 { ref segname, ref sections, ref vmaddr, ref fileoff, .. } = cmd {
                println!("segment: {}", segname);
                println!("vmaddr: 0x{:x}", vmaddr);
                println!("offset: {}", fileoff);

                for ref sect in sections {
                    println!(" section: {}", sect.sectname);
                }
                
            }
        }
        for &MachCommand(ref cmd, cmdsize) in commands {
            if let &LoadCommand::SymTab { ref symoff, ref nsyms, ref stroff, ref strsize} = cmd {
                println!("\n------------- [ SYMBOL INFO ] -------------");
                println!("symbol start offset: 0x{:x}", symoff);
                println!("total symbols: {}", nsyms);
                println!("string start offset: 0x{:x}", stroff);
                println!("string size: {}", strsize);

            }
            if let &LoadCommand::EntryPoint {ref entryoff, ref stacksize } = cmd {
                println!("entry: {:x}", entryoff);
                println!("stack size: {}", stacksize);
            }
        }

    }       
                     
}
