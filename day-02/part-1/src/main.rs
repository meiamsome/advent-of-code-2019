use vm::lang::load_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = load_from_file("./input.txt")?;
    while let Some(_) = vm.next() {};
    println!("{:?}", vm.memory.memory);
    Ok(())
}
