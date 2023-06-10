#[allow(unused)]
#[derive(Debug)]
pub enum VMError {
    OutOfBuffer,
    UnknowedOpcode,
    StackOverflow,
    StackEmpty,
}

impl std::fmt::Display for VMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBuffer    => write!(f, "Out of Buffer"),
            Self::UnknowedOpcode => write!(f, "Unknowed Opcode"),
            Self::StackOverflow  => write!(f, "Stack Overflow"),
            Self::StackEmpty     => write!(f, "Stack Empty"),
        }
    }
}

impl std::error::Error for VMError { }

#[allow(unused)]
pub struct VM {
    index:  u16,
    pc:     u16,
    sp:     u16,
    stack:  [u16; 16],
    regs:   [u8; 16],
    timer:  (u8, u8),
    pub gfx:    [u8; 64 * 32],
    keys:   [u8; 16],
    memory: [u8; 4096],
}

impl Default for VM {
    fn default() -> Self {
        Self {
            index:  0x0000,
            pc:     0x0200,
            sp:     0x0000,
            stack:  [0x0000; 16],
            regs:   [0x00; 16],
            timer:  (0x00, 0x00),
            gfx:    [0x00; 64 * 32],
            keys:   [0x00; 16],
            memory: [0x00; 4096],
        }
    }
}

impl std::fmt::Display for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"
index   : 0x{:04x}
pc      : 0x{:04x}
sp      : 0x{:04x}
V0 ~ V3 : 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}
V4 ~ V7 : 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}
V8 ~ VB : 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}
VC ~ VF : 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}
"#,
    self.index, self.pc, self.sp,
    self.regs[ 0], self.regs[ 1], self.regs[ 2], self.regs[ 3],
    self.regs[ 4], self.regs[ 5], self.regs[ 6], self.regs[ 7],
    self.regs[ 8], self.regs[ 9], self.regs[10], self.regs[11],
    self.regs[12], self.regs[13], self.regs[14], self.regs[15],
        )
    }
}

impl VM {
    #[allow(unused)]
    const fn get_opcode(&self) -> u16 {
          (self.memory[self.pc as usize    ] as u16) << 8
        | (self.memory[self.pc as usize + 1] as u16)
    }

    #[allow(unused)]
    fn update_pc(&mut self) {
        self.pc += 2;
    }

    #[allow(unused)]
    fn push_stack(&mut self, v: u16) -> Result<(), VMError> {
        if self.sp >= 0x0010 {
            Err(VMError::StackOverflow)
        } else {
            self.stack[self.sp as usize] = v;
            self.sp += 1;
            Ok(())
        }
    }

    #[allow(unused)]
    fn pop_stack(&mut self) -> Result<u16, VMError> {
        if self.sp > 0x0000 {
            Err(VMError::StackEmpty)
        } else {
            self.sp -= 1;
            Ok(self.stack[self.sp as usize])
        }
    }
}

impl VM {
    #[allow(unused)]
    fn load<const MAX: usize, const OFFSET: usize>(&mut self, buffer: &[u8]) -> Result<(), VMError> {
        if buffer.len() > MAX {
            Err(VMError::OutOfBuffer)
        } else {
            for i in 0x000..buffer.len() {
                self.memory[i + OFFSET] = buffer[i];
            }
            Ok(())
        }
    }
    #[allow(unused)]
    fn exec_0nnn(&mut self, opcode: u16) -> Result<(), VMError> {
        todo!();
    }

    /// Jump to address `nnn`
    #[allow(unused)]
    fn exec_1nnn(&mut self, opcode: u16) -> Result<(), VMError> {
        self.pc = opcode & 0x0FFF;
        return Ok(());
    }

    /// Execute subroutine starting at address `nnn`
    #[allow(unused)]
    fn exec_2nnn(&mut self, opcode: u16) -> Result<(), VMError> {
        self.push_stack(self.pc)?;
        self.pc = opcode & 0x0FFF;
        return Ok(());
    }

    /// Store number `NN` in register `VX`
    #[allow(unused)]
    fn exec_6xnn(&mut self, opcode: u16) -> Result<(), VMError> {
        let vx = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        self.regs[vx] = nn;
        return Ok(());
    }

    /// **Add** the value of register `VY` to register `VX`
    /// 
    /// Set `VF` to `01` if a carry occurs Set `VF` to `00` if a carry does not occur
    #[allow(unused)]
    fn exec_8xy4(&mut self, vx: usize, vy: usize) -> Result<(), VMError> {
        let (v, is_overflow) = self.regs[vx].overflowing_add(self.regs[vy]);
        self.regs[vy] = v;
        if is_overflow {
            self.regs[0xF] = 0x01;
        } else {
            self.regs[0xF] = 0x00;
        }
        return Ok(());
    }

    /// Store memory address `nnn` in register `index`
    #[allow(unused)]
    fn exec_annn(&mut self, opcode: u16) -> Result<(), VMError> {
        self.index = opcode & 0x0FFF;
        return Ok(())
    }

    /// Draw a sprite at position `VX`, `VY` with `N` bytes of sprite data starting at the address stored in `I`
    /// 
    /// Set `VF` to `01` if any set pixels are changed to unset, and `00` otherwise
    #[allow(unused)]
    fn exec_dnnn(&mut self, opcode: u16) -> Result<(), VMError> {
        let vx   = ((opcode & 0x0F00) >> 8) as usize;
        let vy   = ((opcode & 0x00F0) >> 4) as usize;
        let vx   = self.regs[vx] as usize;
        let vy   = self.regs[vy] as usize;
        let  n   = ((opcode & 0x000F)     ) as usize;
        for i in 0..n {
            let pixel = self.memory[(self.index as usize) + i];
            println!("Pixel: {:08b}", pixel);
            let start_line = vy + i;
            if start_line >= 32 {
                break;
            }
            let start = start_line * 64 + vx;
            for j in 0..8 {
                if vx + j >= 64 {
                    break;
                }
                self.gfx[start + j] = (pixel << j) & 0b10000000;
            }
        }
        // TODO draw flag
        // TODO VF
        return Ok(())
    }

    // Store the binary-coded decimal equivalent of the value stored in register VX at addresses `I`, `I + 1`, and `I + 2`
    #[allow(unused)]
    fn exec_fx33(&mut self, vx: usize) -> Result<(), VMError> {
        // TODO 考虑 self.index 超出内存
        self.memory[(self.index as usize)    ] = self.regs[vx] / 100;
        self.memory[(self.index as usize) + 1] = (self.regs[vx] / 10) % 10;
        self.memory[(self.index as usize) + 2] = (self.regs[vx] / 100) % 10;
        return Ok(());
    }
    
}

impl VM {
    #[allow(unused)]
    fn driv_8000(&mut self, opcode: u16) -> Result<(), VMError> {
        let vx   = ((opcode & 0x0F00) >> 8) as usize;
        let vy   = ((opcode & 0x00F0) >> 4) as usize;
        match opcode & 0x000F {
            4 => self.exec_8xy4(vx, vy),
            _ => Err(VMError::UnknowedOpcode),
        }
    }

    #[allow(unused)]
    fn driv_f000(&mut self, opcode: u16) -> Result<(), VMError> {
        let vx   = ((opcode & 0x0F00) >> 8) as usize;
        match opcode & 0x00FF {
            0x0033 => self.exec_fx33(vx),
            _ => Err(VMError::UnknowedOpcode),
        }
    }

    #[allow(unused)]
    pub fn step(&mut self) -> Result<(), VMError> {
        let opcode = self.get_opcode();
        println!("Opcode: 0x{:04x}", opcode);
        let ret = match opcode & 0xf000 {
            0x1000 => self.exec_1nnn(opcode),
            0x2000 => self.exec_2nnn(opcode),
            0x6000 => self.exec_6xnn(opcode),
            0x8000 => self.driv_8000(opcode),
            0xa000 => self.exec_annn(opcode),
            0xd000 => self.exec_dnnn(opcode),
            0xf000 => self.driv_f000(opcode),
            _ => Err(VMError::UnknowedOpcode),
        };
        self.update_pc();
        if self.timer.0 > 0 {
            self.timer.0 -= 1;
        }
        if self.timer.1 > 0 {
            self.timer.1 -= 1;
        }
        return ret;
    }

    #[allow(unused)]
    pub fn load_rom(&mut self, buffer: &[u8]) -> Result<(), VMError> {
        self.load::<0xDFF, 0x200>(buffer)
    }

    #[allow(unused)]
    pub fn load_fontset(&mut self, buffer: &[u8]) -> Result<(), VMError> {
        self.load::<0x050, 0x000>(buffer)
    }

    #[allow(unused)]
    pub fn get_gfx(&self) -> &[u8] {
        return &self.gfx[..];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1nnn() {
        let mut vm: VM = Default::default();
        vm.load_rom(&[
            0x12, 0xF0,
        ]).unwrap();
        vm.step().unwrap();
        assert_eq!(
            vm.pc, 0x2F0
        );
    }

    #[test]
    fn test_annn() {
        let mut vm: VM = Default::default();
        vm.load_rom(&[
            0xA2, 0xF0,
        ]).unwrap();
        vm.step().unwrap();
        assert_eq!(
            vm.index, 0x2F0
        );
    }

}