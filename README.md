**模拟器（emulator）**：模拟器是一种模拟某种计算系统的内部设计和功能的计算程序，其能够在一种计算系统上执行为另一种计算系统（计算系统或计算架构完全不同）设计的程序

**CHIP-8**：CHIP-8 是一种于 20 世纪 70 年代由 Joseph Weisbecker 开发的一种虚拟机，使用 CHIP-8 语言写成的程序可以运行在任何拥有 CHIP-8 解释器的计算系统上

| bit  |   Register    |              Description               |
| :--: | :-----------: | :------------------------------------: |
|  16  |      `I`      |               索引寄存器               |
|  16  |     `PC`      |               程序计数器               |
|  8   |  `V0 ... VE`  |  通用寄存器 General Purpose Register   |
|  8   |     `VF`      |    标志寄存器，被用作 `carry flag`     |
|  8   | `delay_timer` |                  60Hz                  |
|  8   | `sound_timer` |                  60Hz                  |
|  16  |     `SP`      | 栈顶指示器，栈拥有 16 级 16-bit 的栈区 |

**按键（Key）**：使用 `[u8; 16]` 来映射按键的状态

**内存（Memory）**：CHIP-8 拥有总共 4k 的 8-bit 内存，内存映射如下

```
0x000 - 0x1FF: Chip 8 interpreter(contains font set in emulator)
0x000 - 0x050: fontset
0x050 - 0x0A0: Used for the built in 4*5 pixel and 0x0 ~ 0xF
0x200 - 0xFFF: Program ROM and Work RAM
```

**显存（Video Memory）**：`64 * 32` 个 8-bit 像素，指仅为 `0/1`，称为 `gfx`

**CHIP-8 指令集**：CHIP-8 有 35 个 Opcode，大小均为 2 字节

| Type | Instruction |                          Desciption                          |
| :--: | :---------: | :----------------------------------------------------------: |
|  J   |   `0NNN`    |     Execute machine language subroutine at address `NNN`     |
|  R   |   `00E0`    |                       Clear the screen                       |
|  J   |   `00EE`    |                   Return from a subroutine                   |
|  J   |   `1NNN`    |                    Jump to address `NNN`                     |
|  J   |   `2NNN`    |         Execute subroutine starting at address `NNN`         |
|  J   |   `3XNN`    | Skip the following instruction if the value of register `VX` equals `NN` |
|  J   |   `4XNN`    | Skip the following instruction if the value of register `VX` is not equal to `NN` |
|  J   |   `5XY0`    | Skip the following instruction if the value of register `VX` is equal to the value of register `VY` |
|  M   |   `6XNN`    |              Store number `NN` in register `VX`              |
|  C   |   `7XNN`    |           **Add** the value `NN` to register `VX`            |
|  M   |   `8XY0`    |      Store the value of register `VY` in register `VX`       |
|  C   |   `8XY1`    |                 Set `VX` to `VX` **OR** `VY`                 |
|  C   |   `8XY2`    |                Set `VX` to `VX` **AND** `VY`                 |
|  C   |   `8XY3`    |                Set `VX` to `VX` **XOR** `VY`                 |
|  C   |   `8XY4`    | **Add** the value of register `VY` to register `VX`<br />Set `VF` to `01` if a carry occurs Set `VF` to `00` if a carry does not occur |
|  C   |   `8XY5`    | **Subtract** the value of register `VY` from register `VX`<br />Set `VF` to `00` if a borrow occurs Set `VF` to `01` if a borrow does not occur |
|  C   |   `8XY6`    | Store the value of register `VY` **shifted right** one bit in register `VX`<br />Set register `VF` to the least significant bit prior to the shift `VY` is unchanged |
|  C   |   `8XY7`    | Set register `VX` to the value of `VY` **minus** `VX`<br />Set `VF` to `00` if a borrow occurs Set `VF` to `01` if a borrow does not occur |
|  C   |   `8XYE`    | Store the value of register `VY` **shifted left** one bit in register `VX`<br />Set register `VF` to the most significant bit prior to the shift `VY` is unchanged |
|  J   |   `9XY0`    | Skip the following instruction if the value of register `VX` is not equal to the value of register `VY` |
|  M   |   `ANNN`    |          Store memory address `NNN` in register `I`          |
|  J   |   `BNNN`    |                  Jump to address `NNN + V0`                  |
|  M   |   `CXNN`    |       Set `VX` to a random number with a mask of `NN`        |
|  R   |   `DXYN`    | Draw a sprite at position `VX`, `VY` with `N` bytes of sprite data starting at the address stored in `I`<br />Set `VF` to `01` if any set pixels are changed to unset, and `00` otherwise |
|  J   |   `EX9E`    | Skip the following instruction if the key corresponding to the hex value currently stored in register `VX` is pressed |
|  J   |   `EXA1`    | Skip the following instruction if the key corresponding to the hex value currently stored in register `VX` is not pressed |
|  M   |   `FX07`    | Store the current value of the delay timer in register `VX`  |
|  M   |   `FX0A`    |  Wait for a keypress and store the result in register `VX`   |
|  M   |   `FX15`    |      Set the delay timer to the value of register `VX`       |
|  M   |   `FX18`    |      Set the sound timer to the value of register `VX`       |
|  C   |   `FX1E`    |    Add the value stored in register `VX` to register `I`     |
|  M   |   `FX29`    | Set `I` to the memory address of the sprite data corresponding to the hexadecimal digit stored in register `VX` |
|  M   |   `FX33`    | Store the binary-coded decimal equivalent of the value stored in register VX at addresses `I`, `I + 1`, and `I + 2` |
|  M   |   `FX55`    | Store the values of registers `V0` to `VX` inclusive in memory starting at address `I` `I` is set to `I + X + 1` after operation |
|  M   |   `FX65`    | Fill registers `V0` to `VX` inclusive with the values stored in memory starting at address `I` `I` is set to `I + X + 1` after operation |

```python
self.itable = [
idef(opcode=0x00E0, mask=0xffff, name="CLS",  d=self.decode_OP, cf=0, cmt="Clear video memory"),
idef(opcode=0x00EE, mask=0xffff, name="RET",  d=self.decode_OP, cf=CF_STOP, cmt="Return from subroutine"),
idef(opcode=0x0000, mask=0xf000, name="SYS",  d=self.decode_NNN_mem, cf=CF_USE1, cmt="Call CDP1802 subroutine at NNN"),
idef(opcode=0x1000, mask=0Xf000, name="JP",   d=self.decode_NNN_mem, cf=CF_USE1|CF_JUMP, cmt="Jump to address NNN"),
idef(opcode=0x2000, mask=0xf000, name="CALL", d=self.decode_NNN_mem, cf=CF_USE1|CF_CALL, cmt="Call CHIP-8 subroutine at NNN"),
idef(opcode=0x3000, mask=0xf000, name="SE",   d=self.decode_XNN, cf=CF_USE1|CF_USE2|CF_JUMP, cmt="Skip next instruction if VX == NN"),
idef(opcode=0x4000, mask=0xf000, name="SNE",  d=self.decode_XNN, cf=CF_USE1|CF_USE2|CF_JUMP, cmt="Skip next instruction if VX != NN"),
idef(opcode=0x5000, mask=0xf00f, name="SE",   d=self.decode_XY, cf=CF_USE1|CF_USE2|CF_JUMP, cmt="Skip next instruction if VX == VY"),
idef(opcode=0x6000, mask=0xf000, name="LD",   d=self.decode_XNN, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = NN"),
idef(opcode=0x7000, mask=0xf000, name="ADD",  d=self.decode_XNN, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = VX + NN"),
idef(opcode=0x8000, mask=0xf00f, name="LD",   d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = VY"),
idef(opcode=0x8001, mask=0xf00f, name="OR",   d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = VX OR VY"),
idef(opcode=0x8002, mask=0xf00f, name="AND",  d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = VX AND VY"),
idef(opcode=0x8003, mask=0xf00f, name="XOR",  d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = VX XOR VY"),
idef(opcode=0x8004, mask=0xf00f, name="ADD",  d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = VX + VY; VF = 1 if overflow else 0"),
idef(opcode=0x8005, mask=0xf00f, name="SUB",  d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = VX - VY; VF = 1 if not borrow else 0"),
idef(opcode=0x8006, mask=0xf00f, name="SHR",  d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VF = LSB(VX); VX = VX » 1 (** see note)"),
idef(opcode=0x8007, mask=0xf00f, name="SUBN", d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = VY - VX; VF = 1 if not borrow else 0"),
idef(opcode=0x800E, mask=0xf00f, name="SHL",  d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VF = MSB(VX); VX = VX « 1 (** see note)"),
idef(opcode=0x9000, mask=0xf00f, name="SNE",  d=self.decode_XY, cf=CF_USE1|CF_USE2|CF_JUMP, cmt="Skip next instruction if VX != VY"),
idef(opcode=0xA000, mask=0xf000, name="LD",   d=self.decode_LD_I, cf=CF_USE1, cmt="I = NNN"),
idef(opcode=0xB000, mask=0xf000, name="JP",   d=self.decode_JP_V0, cf=CF_USE1|CF_JUMP, cmt="Jump to address NNN + V0"),
idef(opcode=0xC000, mask=0xf000, name="RND",  d=self.decode_XY, cf=CF_USE1|CF_CHG1|CF_USE2, cmt="VX = RND() AND NN"),
idef(opcode=0xD000, mask=0xf000, name="DRW",  d=self.decode_XYN, cf=CF_USE1|CF_USE2|CF_USE3, cmt="Draw 8xN sprite at I to VX, VY; VF = 1 if collision else 0"),
idef(opcode=0xE09E, mask=0xf0ff, name="SKP",  d=self.decode_X, cf=CF_USE1|CF_JUMP, cmt="Skip next instruction if key(VX) is pressed"),
idef(opcode=0xE0A1, mask=0xf0ff, name="SKNP", d=self.decode_X, cf=CF_USE1|CF_JUMP, cmt="Skip next instruction if key(VX) is not pressed"),
idef(opcode=0xF007, mask=0xf0ff, name="LD",   d=self.decode_LD_VX_DT, cf=CF_USE1|CF_CHG1, cmt="LD VX, DT;Sets VX to the value of the delay timer."),
idef(opcode=0xF00A, mask=0xf0ff, name="LD",   d=self.decode_LD_K, cf=CF_USE1|CF_CHG1, cmt="Wait for key press, store key pressed in VX"),
idef(opcode=0xF015, mask=0xf0ff, name="LD",   d=self.decode_LD_DT_VX, cf=CF_USE1, cmt="DT = VX;Sets the delay timer to VX."),
idef(opcode=0xF018, mask=0xf0ff, name="LD",   d=self.decode_LD_ST, cf=CF_USE1, cmt="ST = VX;Sets the sound timer to VX."),
idef(opcode=0xF01E, mask=0xf0ff, name="ADD",  d=self.decode_ADD_I, cf=CF_USE1, cmt="I = I + VX; VF = 1 if I > 0xFFF else 0"),
idef(opcode=0xF029, mask=0xf0ff, name="LD",   d=None, cf=CF_USE1, cmt="I = address of 4x5 font character in VX (0..F) (* see note)"),
idef(opcode=0xF033, mask=0xf0ff, name="BCD",  d=self.decode_X, cf=CF_USE1, cmt="set_BCD(Vx);*(I+0)=BCD(3);*(I+1)=BCD(2);*(I+2)=BCD(1);Store BCD representation of VX at I (100), I+1 (10), and I+2 (1); I remains unchanged"),
idef(opcode=0xF055, mask=0xf0ff, name="LD",   d=self.decode_STORE_I, cf=CF_USE1, cmt="Store V0..VX (inclusive) to memory starting at I; I remains unchanged"),
idef(opcode=0xF065, mask=0xf0ff, name="LD",   d=self.decode_LOAD_I, cf=CF_USE1, cmt="Load V0..VX (inclusive) from memory starting at I; I remains unchanged")
    ]
```

