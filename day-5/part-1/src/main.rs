use vm::lang::load_from_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vm = load_from_file("./input.txt")?;
    let last_memory = vm.last().unwrap();
    println!("{:?}", last_memory);
    Ok(())
}
