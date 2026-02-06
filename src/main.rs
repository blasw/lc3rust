use std::env::args;
use std::fs::File;
use std::io::BufReader;
use crate::hardware::vm::VM;
use crate::utils::U16FileReader;

mod hardware;
mod utils;

fn main() {
    let path = args().nth(1).unwrap_or("./hello-world.obj".to_string());
    let app = File::open(path).expect("Failed to open application file");

    let mut buf = U16FileReader::new(BufReader::new(app));

    let mut vm = VM::new();

    let base_addr = buf.read_u16().unwrap();
    let mut addr = base_addr as usize;
    loop {
        match buf.read_u16() {
            Ok(instruction) => {
                println!("{} - {}", addr, instruction);
                vm.write_memory(addr as u16, instruction);

                addr += 1;
            }
            Err(e) => {
                if let Some(io_err) = e.downcast_ref::<std::io::Error>() && io_err.kind() ==
                    std::io::ErrorKind::UnexpectedEof {
                        println!("OK");
                } else {
                    println!("Error: {}", e);
                }
                break;
            }
        }
    }
    println!("Executing now");
    vm.execute();
}
