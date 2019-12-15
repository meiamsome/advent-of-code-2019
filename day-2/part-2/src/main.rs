use vm::lang::{get_ops, load_memory_from_file};
use vm::IntcodeVM;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vm_memory = load_memory_from_file("./input.txt")?;
    for x in 0..=99 {
        for y in 0..=99 {
            let mut vm = IntcodeVM::create(vm_memory.clone(), get_ops());
            vm.memory[1] = x;
            vm.memory[2] = y;
            let last_memory = vm.last().unwrap();
            if last_memory[0] == 19690720 {
                println!("{:?}", last_memory);
                println!("{}", 100 * x + y);
                return Ok(())
            }
        }
    }
    println!("No solution found!");
    Ok(())
}
