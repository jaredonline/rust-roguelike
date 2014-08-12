static INST_SET_HEALTH      : u8 = 0u8;
static INST_SET_WISDOM      : u8 = 1u8;
static INST_SET_AGILITY     : u8 = 2u8;
static INST_PLAY_SOUND      : u8 = 3u8;
static INST_SPAWN_PARTICLES : u8 = 4u8;
static INST_LITERAL         : u8 = 5u8;

pub struct VM {
    max_stack_size: int,
    stack_size:     int,
    stack:          Vec<int>
}

impl VM {
    pub fn new() -> VM {
        let mut stack : Vec<int> = vec![];
        VM { max_stack_size: 128, stack_size: 0, stack: stack }
    }

    pub fn interpret(&mut self, array: &[u8]) {
        let mut i = 0;
        let size = array.len();
        loop {
            let num = match array.get(i) {
                Some(n) => *n,
                None    => fail!("Array out of bounds")
            };
            i = self.parse_code(i, num, array);
            i += 1;
            if (i >= size) { break; }
        }
    }

    fn parse_code(&mut self, index: uint, code: u8, array: &[u8]) -> uint {
        let mut i = index;
        match code {
            INST_SET_HEALTH => { 
                let health = self.pop();
                println!("set health to {:d}", health);
            },
            INST_SET_WISDOM => {
                let wisdom = self.pop();
                println!("set wisdom to {:d}", wisdom);
            },
            INST_LITERAL    => {
                i += 1;
                let value = match array.get(i) {
                    Some(n) => *n as int,
                    None    => fail!("No matching value for literal")
                };
                self.push(value);
            },
            _               => fail!("Unsupported byte code encountered")
        };
        return i;
    }

    fn push(&mut self, value: int) {
        if self.stack_size > self.max_stack_size {
            fail!("Stack overflow.");
        } else {
            self.stack_size += 1;
            self.stack.push(value);
        }
    }
    
    fn pop(&mut self) -> int {
        match self.stack.pop() {
            Some(n) => { self.stack_size -= 1; return n; },
            None    => fail!("Stack empty.")
        }
    }
}
