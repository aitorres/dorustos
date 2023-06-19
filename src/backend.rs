use rand::random;

/// Width of the screen in pixels (before any scaling is applied)
pub const SCREEN_WIDTH: usize = 64;

/// Height of the screen in pixels (before any scaling is applied)
pub const SCREEN_HEIGHT: usize = 32;

/// Total amount of pixels used in the screen
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

/// Total amount of bytes used in the RAM
const RAM_SIZE: usize = 4096;

/// Total amount of registers (V0 to VF)
const NUM_REGS: usize = 16;

/// Total amount of stack levels
const STACK_SIZE: usize = 16;

/// Starting address of the program
const START_ADDR: u16 = 0x200;

/// Total amount of keys in the keypad
const NUM_KEYS: usize = 16;

/// Amount of bytes used for the fontset
const FONTSET_SIZE: usize = 80;

/// Chip-8 fontset
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

/// A Chip8 virtual machine implementation
pub struct Chip8 {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_SIZE],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Chip8 {
    /// Returns a new instance of the Chip-8 virtual machine with sensible
    /// default values
    pub fn new() -> Self {
        let mut chip8 = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_SIZE],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        chip8.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        chip8
    }

    /// Performs one CPU tick on the Chip-8 virtual machine.
    /// Multiple CPU ticks can happen on a single frame.
    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();

        // Decode and execute
        self.execute(op);
    }

    /// Performs one timer tick on the Chip-8 virtual machine.
    /// This should happen once per frame.
    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // TODO: implement BEEP
            }
            self.st -= 1;
        }
    }

    /// Returns a slice of the screen buffer
    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    /// Registers a keypress in the keypad
    ///
    /// # Arguments
    ///
    /// * `idx` - Index of the key in the keypad
    /// * `pressed` - Whether the key was pressed or released
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    /// Loads a program into the Chip-8 virtual machine
    ///
    /// # Arguments
    ///
    /// * `data` - The program to load into the virtual machine
    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = start + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    /// Returns the operation code of the next instruction to execute
    /// according to the program counter.
    /// Note that each instruction is 2 bytes long, stored in the RAM
    /// as part of the loaded program.
    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }

    /// Executes an operation on the Chip-8 virtual machine and updates the
    /// machine's state accordingly.
    ///
    /// # Arguments
    ///
    /// * `op` - The operation code to execute
    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => self.screen = [false; SCREEN_SIZE],
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }
            (1, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.pc = nnn;
            }
            (2, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.push(self.pc);
                self.pc = nnn;
            }
            (3, _, _, _) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let nn = (op & 0x00FF) as u8;
                if vx == nn {
                    self.pc += 2;
                };
            }
            (4, _, _, _) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let nn = (op & 0x00FF) as u8;
                if vx != nn {
                    self.pc += 2;
                };
            }
            (5, _, _, 0) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let y = digit3 as usize;
                let vy = self.v_reg[y];

                if vx == vy {
                    self.pc += 2;
                };
            }
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;

                self.v_reg[x] = nn;
            }
            (7, _, _, _) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let nn = (op & 0x00FF) as u8;

                self.v_reg[x] = vx.wrapping_add(nn);
            }
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                let vy = self.v_reg[y];

                self.v_reg[x] = vy;
            }
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let y = digit3 as usize;
                let vy = self.v_reg[y];

                self.v_reg[x] = vx | vy;
            }
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let y = digit3 as usize;
                let vy = self.v_reg[y];

                self.v_reg[x] = vx & vy;
            }
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let y = digit3 as usize;
                let vy = self.v_reg[y];

                self.v_reg[x] = vx ^ vy;
            }
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let y = digit3 as usize;
                let vy = self.v_reg[y];

                let (new_vx, carry) = vx.overflowing_add(vy);
                let new_vf = if carry { 1 } else { 0 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let y = digit3 as usize;
                let vy = self.v_reg[y];

                let (new_vx, borrow) = vx.overflowing_sub(vy);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let lsb = vx & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let y = digit3 as usize;
                let vy = self.v_reg[y];

                let (new_vx, borrow) = vy.overflowing_sub(vx);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let msb = (vx >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];

                let y = digit3 as usize;
                let vy = self.v_reg[y];

                if vx != vy {
                    self.pc += 2;
                };
            }
            (0xA, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.i_reg = nnn;
            }
            (0xB, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            }
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            }
            (0xD, _, _, _) => {
                // Get (x, y) coords for the sprite
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;

                // Get the height of the sprite
                let num_rows = digit4;

                // Keep track of whether we've flipped a pixel
                let mut flipped = false;

                // Iterate over each row of the sprite
                for y_line in 0..num_rows {
                    // Check which memory address our row's data is stored on
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    // Iterate over each column in our row (rows are 8 bits long)
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit and only flip if it's 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get pixel's index for our 1D screen array
                            let idx = x + SCREEN_WIDTH * y;

                            // Check if we're about to flip and set the new value
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            }
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];

                if key {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];

                if !key {
                    self.pc += 2;
                }
            }
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.v_reg[x] = self.dt;
            }
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            }
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.dt = self.v_reg[x];
            }
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.st = self.v_reg[x];
            }
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            }
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.v_reg[x] as u16;
                self.i_reg = c * 5;
            }
            (0xF, _, 3, 3) => {
                // TODO: optimize?
                let x = digit2 as usize;
                let vx = self.v_reg[x] as f32;

                let hundreds = (vx * 100.0).floor() as u8;
                let tens = ((vx * 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            }
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            }
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:X}", op),
        };
    }

    /// Pushes a new value onto the machine's stack
    ///
    /// # Arguments
    ///
    /// * `val` - The value to push onto the stack
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    /// Pops and returns a value off the machine's stack
    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
