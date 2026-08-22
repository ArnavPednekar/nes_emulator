struct CpuFlags {}

pub struct CPU {
    register_a: u8,
    register_x: u8,

    pc: u16,
    //   sp: u8,
    status: u8,
    memory: [u8; 0xFFFF],
}

impl CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run();
    }
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.pc = 0x8000;
    }
    fn run(&mut self) {
        loop {
            let opscode = program[self.pc as usize];
            self.pc += 1;

            match opscode {
                0xA9 => {
                    let param = program[self.pc as usize];
                    self.pc += 1;

                    self.lda(param);
                }
                0xAA => self.tax(),

                0x00 => return,

                _ => todo!(),
            }
        }
        fn new() -> Self {
            CPU {
                register_a: 0,
                status: 0,
                pc: 0,
                register_x: 0,
            }
        }
        fn lda(&mut self, value: u8) {
            self.register_a = value;
            self.update_zero_and_negative_flags(self.register_a);
        }
        fn tax(&mut self) {
            self.register_x = self.register_a;
            self.update_zero_and_negative_flags(self.register_x);
        }
        fn update_zero_and_negative_flags(&mut self, result: u8) {
            if result == 0 {
                self.status = self.status | 0b0000_0010;
            } else {
                self.status = self.status & 0b1111_1101;
            }

            if self.register_a & 0b1000_0000 != 0 {
                self.status = self.status | 0b1000_0000;
            } else {
                self.status = self.status & 0b0111_1111;
            }
        }
        fn interpret(&mut self, program: Vec<u8>) {
            self.pc = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
