use vm::lang::{get_ops, load_memory_from_file, op_code_lookup};
use vm::IntcodeVM;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vm_memory = load_memory_from_file("./input.txt")?;
    for x in 0..=99 {
        for y in 0..=99 {
            let mut vm =
                IntcodeVM::create(vm_memory.clone(), get_ops(), &op_code_lookup, None, None);
            vm.memory.memory[1] = x;
            vm.memory.memory[2] = y;
            while let Some(_) = vm.next() {}
            if vm.memory.memory[0] == 19_690_720 {
                println!("{:?}", vm.memory.memory);
                println!("{}", 100 * x + y);
                return Ok(());
            }
        }
    }
    println!("No solution found!");
    Ok(())
}
