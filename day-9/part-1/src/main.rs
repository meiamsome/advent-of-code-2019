use vm::io::create_stdio_vmio;
use vm::lang::load_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = load_from_file("./input.txt")?;
    vm.io = create_stdio_vmio();
    vm.last().unwrap();
    Ok(())
}
