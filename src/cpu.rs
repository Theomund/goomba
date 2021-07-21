pub struct Cpu {
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    program_counter: u16,
    stack_pointer: u8,
    processor_status: u8,
    memory: [u8; 0xFF],
}

pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

pub enum Instruction {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

pub struct Opcode {
    instruction: Instruction,
    mode: AddressingMode,
    bytes: u8,
    cycles: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            program_counter: 0x34,
            stack_pointer: 0xFD,
            processor_status: 0,
            memory: [0; 0xFF],
        }
    }

    pub fn run(&mut self) {
        let address = self.fetch();
        let opcode = self.decode(address);
        self.execute(opcode)
    }

    fn fetch(&mut self) -> u8 {
        self.memory[self.program_counter as usize]
    }

    fn decode(&mut self, address: u8) -> Opcode {
        match address {
            0x00 => Opcode {
                instruction: Instruction::Brk,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 7,
            },
            0x01 => Opcode {
                instruction: Instruction::Ora,
                mode: AddressingMode::IndexedIndirect,
                bytes: 2,
                cycles: 6,
            },
            0x05 => Opcode {
                instruction: Instruction::Ora,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0x06 => Opcode {
                instruction: Instruction::Asl,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            },
            0x09 => Opcode {
                instruction: Instruction::Ora,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0x0A => Opcode {
                instruction: Instruction::Asl,
                mode: AddressingMode::Accumulator,
                bytes: 1,
                cycles: 2,
            },
            0x0D => Opcode {
                instruction: Instruction::Ora,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0x0E => Opcode {
                instruction: Instruction::Asl,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            },
            0x10 => Opcode {
                instruction: Instruction::Bpl,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            },
            0x11 => Opcode {
                instruction: Instruction::Ora,
                mode: AddressingMode::IndirectIndexed,
                bytes: 2,
                cycles: 5,
            },
            0x15 => Opcode {
                instruction: Instruction::Ora,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 4,
            },
            0x16 => Opcode {
                instruction: Instruction::Asl,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            },
            0x18 => Opcode {
                instruction: Instruction::Clc,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0x19 => Opcode {
                instruction: Instruction::Ora,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            },
            0x1D => Opcode {
                instruction: Instruction::Ora,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            },
            0x1E => Opcode {
                instruction: Instruction::Asl,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            },
            0x20 => Opcode {
                instruction: Instruction::Jsr,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            },
            0x21 => Opcode {
                instruction: Instruction::And,
                mode: AddressingMode::IndexedIndirect,
                bytes: 2,
                cycles: 6,
            },
            0x24 => Opcode {
                instruction: Instruction::Bit,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0x25 => Opcode {
                instruction: Instruction::And,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0x29 => Opcode {
                instruction: Instruction::And,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0x2C => Opcode {
                instruction: Instruction::Bit,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0x2D => Opcode {
                instruction: Instruction::And,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0x30 => Opcode {
                instruction: Instruction::Bmi,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            },
            0x31 => Opcode {
                instruction: Instruction::And,
                mode: AddressingMode::IndirectIndexed,
                bytes: 2,
                cycles: 5,
            },
            0x35 => Opcode {
                instruction: Instruction::And,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            },
            0x38 => Opcode {
                instruction: Instruction::Sec,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0x39 => Opcode {
                instruction: Instruction::And,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            },
            0x3D => Opcode {
                instruction: Instruction::And,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            },
            0x41 => Opcode {
                instruction: Instruction::Eor,
                mode: AddressingMode::IndexedIndirect,
                bytes: 2,
                cycles: 6,
            },
            0x45 => Opcode {
                instruction: Instruction::Eor,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0x46 => Opcode {
                instruction: Instruction::Lsr,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            },
            0x49 => Opcode {
                instruction: Instruction::Eor,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0x4A => Opcode {
                instruction: Instruction::Lsr,
                mode: AddressingMode::Accumulator,
                bytes: 1,
                cycles: 2,
            },
            0x4C => Opcode {
                instruction: Instruction::Jmp,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 3,
            },
            0x4D => Opcode {
                instruction: Instruction::Eor,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0x4E => Opcode {
                instruction: Instruction::Lsr,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            },
            0x50 => Opcode {
                instruction: Instruction::Bvc,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            },
            0x51 => Opcode {
                instruction: Instruction::Eor,
                mode: AddressingMode::IndirectIndexed,
                bytes: 2,
                cycles: 5,
            },
            0x55 => Opcode {
                instruction: Instruction::Eor,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            },
            0x56 => Opcode {
                instruction: Instruction::Lsr,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            },
            0x58 => Opcode {
                instruction: Instruction::Cli,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0x59 => Opcode {
                instruction: Instruction::Eor,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            },
            0x5D => Opcode {
                instruction: Instruction::Eor,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            },
            0x5E => Opcode {
                instruction: Instruction::Lsr,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            },
            0x61 => Opcode {
                instruction: Instruction::Adc,
                mode: AddressingMode::IndexedIndirect,
                bytes: 2,
                cycles: 6,
            },
            0x65 => Opcode {
                instruction: Instruction::Adc,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0x69 => Opcode {
                instruction: Instruction::Adc,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0x6C => Opcode {
                instruction: Instruction::Jmp,
                mode: AddressingMode::Indirect,
                bytes: 3,
                cycles: 5,
            },
            0x6D => Opcode {
                instruction: Instruction::Adc,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0x70 => Opcode {
                instruction: Instruction::Bvs,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            },
            0x71 => Opcode {
                instruction: Instruction::Adc,
                mode: AddressingMode::IndirectIndexed,
                bytes: 2,
                cycles: 5,
            },
            0x75 => Opcode {
                instruction: Instruction::Adc,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            },
            0x78 => Opcode {
                instruction: Instruction::Sei,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0x79 => Opcode {
                instruction: Instruction::Adc,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            },
            0x7D => Opcode {
                instruction: Instruction::Adc,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            },
            0x88 => Opcode {
                instruction: Instruction::Dey,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0x8A => Opcode {
                instruction: Instruction::Txa,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0x90 => Opcode {
                instruction: Instruction::Bcc,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            },
            0x98 => Opcode {
                instruction: Instruction::Tya,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0x9A => Opcode {
                instruction: Instruction::Txs,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xA0 => Opcode {
                instruction: Instruction::Ldy,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0xA1 => Opcode {
                instruction: Instruction::Lda,
                mode: AddressingMode::IndexedIndirect,
                bytes: 2,
                cycles: 6,
            },
            0xA2 => Opcode {
                instruction: Instruction::Ldx,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0xA4 => Opcode {
                instruction: Instruction::Ldy,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0xA5 => Opcode {
                instruction: Instruction::Lda,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0xA6 => Opcode {
                instruction: Instruction::Ldx,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0xA8 => Opcode {
                instruction: Instruction::Tay,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xA9 => Opcode {
                instruction: Instruction::Lda,
                mode: AddressingMode::Absolute,
                bytes: 2,
                cycles: 2,
            },
            0xAA => Opcode {
                instruction: Instruction::Tax,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xAC => Opcode {
                instruction: Instruction::Ldy,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0xAD => Opcode {
                instruction: Instruction::Lda,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0xAE => Opcode {
                instruction: Instruction::Ldx,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0xB0 => Opcode {
                instruction: Instruction::Bcs,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            },
            0xB1 => Opcode {
                instruction: Instruction::Lda,
                mode: AddressingMode::IndirectIndexed,
                bytes: 2,
                cycles: 5,
            },
            0xB4 => Opcode {
                instruction: Instruction::Ldy,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            },
            0xB5 => Opcode {
                instruction: Instruction::Lda,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 4,
            },
            0xB6 => Opcode {
                instruction: Instruction::Ldx,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            },
            0xB8 => Opcode {
                instruction: Instruction::Clv,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xB9 => Opcode {
                instruction: Instruction::Lda,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            },
            0xBA => Opcode {
                instruction: Instruction::Tsx,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xBC => Opcode {
                instruction: Instruction::Ldy,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            },
            0xBD => Opcode {
                instruction: Instruction::Lda,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            },
            0xBE => Opcode {
                instruction: Instruction::Ldx,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            },
            0xC0 => Opcode {
                instruction: Instruction::Cpy,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0xC1 => Opcode {
                instruction: Instruction::Cmp,
                mode: AddressingMode::IndexedIndirect,
                bytes: 2,
                cycles: 6,
            },
            0xC4 => Opcode {
                instruction: Instruction::Cpy,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0xC5 => Opcode {
                instruction: Instruction::Cmp,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0xC6 => Opcode {
                instruction: Instruction::Dec,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            },
            0xC8 => Opcode {
                instruction: Instruction::Iny,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xC9 => Opcode {
                instruction: Instruction::Cmp,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0xCA => Opcode {
                instruction: Instruction::Dex,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xCC => Opcode {
                instruction: Instruction::Cpy,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0xCD => Opcode {
                instruction: Instruction::Cmp,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0xCE => Opcode {
                instruction: Instruction::Dec,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            },
            0xD0 => Opcode {
                instruction: Instruction::Bne,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            },
            0xD1 => Opcode {
                instruction: Instruction::Cmp,
                mode: AddressingMode::IndirectIndexed,
                bytes: 2,
                cycles: 5,
            },
            0xD5 => Opcode {
                instruction: Instruction::Cmp,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            },
            0xD6 => Opcode {
                instruction: Instruction::Dec,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            },
            0xD8 => Opcode {
                instruction: Instruction::Cld,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xD9 => Opcode {
                instruction: Instruction::Cmp,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            },
            0xDD => Opcode {
                instruction: Instruction::Cmp,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            },
            0xDE => Opcode {
                instruction: Instruction::Dec,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            },
            0xE0 => Opcode {
                instruction: Instruction::Cpx,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            },
            0xE4 => Opcode {
                instruction: Instruction::Cpx,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            },
            0xE6 => Opcode {
                instruction: Instruction::Inc,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            },
            0xE8 => Opcode {
                instruction: Instruction::Inx,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xEA => Opcode {
                instruction: Instruction::Nop,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xEC => Opcode {
                instruction: Instruction::Cpx,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            },
            0xEE => Opcode {
                instruction: Instruction::Inc,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            },
            0xF0 => Opcode {
                instruction: Instruction::Beq,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            },
            0xF6 => Opcode {
                instruction: Instruction::Inc,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            },
            0xF8 => Opcode {
                instruction: Instruction::Sed,
                mode: AddressingMode::Implicit,
                bytes: 1,
                cycles: 2,
            },
            0xFE => Opcode {
                instruction: Instruction::Inc,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            },
            _ => todo!(),
        }
    }

    fn execute(&mut self, opcode: Opcode) {
        let parameter = match opcode.mode {
            AddressingMode::Implicit => 0,
            AddressingMode::Relative => {
                self.program_counter += 1;
                self.memory[self.program_counter as usize]
            }
            _ => todo!(),
        };

        match opcode.instruction {
            Instruction::Bcc => self.branch_if_carry_clear(parameter),
            Instruction::Bcs => self.branch_if_carry_set(parameter),
            Instruction::Beq => self.branch_if_equal(parameter),
            Instruction::Bmi => self.branch_if_minus(parameter),
            Instruction::Bne => self.branch_if_not_equal(parameter),
            Instruction::Bpl => self.branch_if_positive(parameter),
            Instruction::Brk => self.force_interrupt(),
            Instruction::Bvc => self.branch_if_overflow_clear(parameter),
            Instruction::Bvs => self.branch_if_overflow_set(parameter),
            Instruction::Clc => self.clear_carry_flag(),
            Instruction::Cld => self.clear_decimal_mode(),
            Instruction::Cli => self.clear_interrupt_disable(),
            Instruction::Clv => self.clear_overflow_flag(),
            Instruction::Dex => self.decrement_x_register(),
            Instruction::Dey => self.decrement_y_register(),
            Instruction::Inx => self.increment_x_register(),
            Instruction::Iny => self.increment_y_register(),
            Instruction::Nop => self.no_operation(),
            Instruction::Sec => self.set_carry_flag(),
            Instruction::Sed => self.set_decimal_flag(),
            Instruction::Sei => self.set_interrupt_disable(),
            Instruction::Tax => self.transfer_accumulator_to_x(),
            Instruction::Tay => self.transfer_accumulator_to_y(),
            Instruction::Tsx => self.transfer_stack_pointer_to_x(),
            Instruction::Txa => self.transfer_x_to_accumulator(),
            Instruction::Txs => self.transfer_x_to_stack_pointer(),
            Instruction::Tya => self.transfer_y_to_accumulator(),
            _ => todo!(),
        }
    }

    fn decrement_x_register(&mut self) {
        self.index_x -= 1;
        self.update_zero_flag(self.index_x);
        self.update_negative_flag(self.index_x);
    }

    fn decrement_y_register(&mut self) {
        self.index_y -= 1;
        self.update_zero_flag(self.index_y);
        self.update_negative_flag(self.index_y);
    }

    fn increment_x_register(&mut self) {
        self.index_x += 1;
        self.update_zero_flag(self.index_x);
        self.update_negative_flag(self.index_x);
    }

    fn increment_y_register(&mut self) {
        self.index_y += 1;
        self.update_zero_flag(self.index_y);
        self.update_negative_flag(self.index_y);
    }

    fn force_interrupt(&mut self) {
        self.processor_status |= 0b0011_0000;
    }

    fn clear_carry_flag(&mut self) {
        self.processor_status &= 0b1111_1110;
    }

    fn clear_decimal_mode(&mut self) {
        self.processor_status &= 0b1111_0111;
    }

    fn clear_interrupt_disable(&mut self) {
        self.processor_status &= 0b1111_1011;
    }

    fn clear_overflow_flag(&mut self) {
        self.processor_status &= 0b1011_1111;
    }

    fn no_operation(&mut self) {}

    fn set_carry_flag(&mut self) {
        self.processor_status |= 0b0000_0001;
    }

    fn set_decimal_flag(&mut self) {
        self.processor_status |= 0b0000_1000;
    }

    fn set_interrupt_disable(&mut self) {
        self.processor_status |= 0b0000_0100;
    }

    fn transfer_accumulator_to_x(&mut self) {
        self.index_x = self.accumulator;
        self.update_zero_flag(self.index_x);
        self.update_negative_flag(self.index_x);
    }

    fn transfer_accumulator_to_y(&mut self) {
        self.index_y = self.accumulator;
        self.update_zero_flag(self.index_y);
        self.update_negative_flag(self.index_y);
    }

    fn transfer_stack_pointer_to_x(&mut self) {
        self.index_x = self.stack_pointer;
        self.update_zero_flag(self.index_x);
        self.update_negative_flag(self.index_x);
    }

    fn transfer_x_to_accumulator(&mut self) {
        self.accumulator = self.index_x;
        self.update_zero_flag(self.accumulator);
        self.update_negative_flag(self.accumulator);
    }

    fn transfer_x_to_stack_pointer(&mut self) {
        self.stack_pointer = self.index_x;
    }

    fn transfer_y_to_accumulator(&mut self) {
        self.accumulator = self.index_y;
        self.update_zero_flag(self.accumulator);
        self.update_negative_flag(self.accumulator);
    }

    fn branch_if_carry_clear(&mut self, displacement: u8) {
        if self.processor_status & 0b0000_0001 == 0 {
            self.program_counter += displacement as u16;
        }
    }

    fn branch_if_carry_set(&mut self, displacement: u8) {
        if self.processor_status & 0b0000_0001 != 0 {
            self.program_counter += displacement as u16;
        }
    }

    fn branch_if_equal(&mut self, displacement: u8) {
        if self.processor_status & 0b0000_0010 != 0 {
            self.program_counter += displacement as u16;
        }
    }

    fn branch_if_minus(&mut self, displacement: u8) {
        if self.processor_status & 0b1000_0000 != 0 {
            self.program_counter += displacement as u16;
        }
    }

    fn branch_if_not_equal(&mut self, displacement: u8) {
        if self.processor_status & 0b0000_0010 == 0 {
            self.program_counter += displacement as u16;
        }
    }

    fn branch_if_positive(&mut self, displacement: u8) {
        if self.processor_status & 0b1000_0000 == 0 {
            self.program_counter += displacement as u16;
        }
    }

    fn branch_if_overflow_clear(&mut self, displacement: u8) {
        if self.processor_status & 0b0100_0000 == 0 {
            self.program_counter += displacement as u16;
        }
    }

    fn branch_if_overflow_set(&mut self, displacement: u8) {
        if self.processor_status & 0b0100_0000 != 0 {
            self.program_counter += displacement as u16;
        }
    }

    fn update_zero_flag(&mut self, operand: u8) {
        if operand == 0 {
            self.processor_status |= 0b0000_0010;
        } else {
            self.processor_status &= 0b1111_1101;
        }
    }

    fn update_negative_flag(&mut self, operand: u8) {
        if operand & 0b1000_0000 != 0 {
            self.processor_status |= 0b1000_0000;
        } else {
            self.processor_status &= 0b0111_1111;
        }
    }
}
