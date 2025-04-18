// https://github.com/aquova/chip8-book/blob/master/src/5-instr.md

use derive_more::{Display, Error};
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const STACK_SIZE: usize = 16;
const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

#[derive(Debug, Display, Error)]
enum StackError {
    StackOverflow,
    StackUnderflow,
}

struct Stack {
    max: Option<usize>,
    data: Vec<u16>,
}

impl Stack {
    fn new(max: Option<usize>) -> Self {
        Self { max, data: vec![] }
    }

    fn pop(&mut self) -> Result<u16, StackError> {
        match self.data.pop() {
            None => Err(StackError::StackUnderflow),
            Some(val) => Ok(val),
        }
    }

    fn push(&mut self, item: u16) -> Result<(), StackError> {
        if let Some(max) = self.max {
            if self.data.len() > max {
                return Err(StackError::StackOverflow);
            }
        }
        self.data.push(item);
        Ok(())
    }

    fn top(self) -> Option<u16> {
        self.data.last().cloned()
    }
}

struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    stack: Stack,
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Emu {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            stack: Stack::new(Some(STACK_SIZE)),
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;

        self.ram = [0; RAM_SIZE];
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.stack = Stack::new(Some(STACK_SIZE));
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                todo!();
            }
            self.st -= 1;
        }
    }
    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode
        // Execute
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize];
        let lower_byte = self.ram[(self.pc + 1) as usize];

        let op = (higher_byte as u16) << 8 | (lower_byte as u16);
        self.pc += 2;
        op
    }
}

fn main() {
    println!("Hello, world!");
}
