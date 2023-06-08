use chip8_rs::{
    VM, VMError
};

fn run() -> Result<(), VMError> {
    let mut vm: VM = Default::default();
    vm.load_rom(&[
        0xA2, 0xF0,
        0x6F, 0x77,
    ])?;
    vm.step()?;
    vm.step()?;
    // vm.step()?;
    // 如何停机
    println!("{}", vm);
    return Ok(());
}

fn main() {
    match run() {
        Err(err) => println!("{}", err),
        Ok(())   => println!("End"),
    }
}