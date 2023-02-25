use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Processor {
    pub x: isize,
    pub ops: usize,
    pub signal_strength: Vec<isize>,
    pub crt_buffer: Vec<char>,
    pub pixel: isize,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            ..Default::default()
        }
    }

    pub fn parse(&mut self, line: String) {
        for op in line.split_whitespace() {
            if self.pixel == self.x - 1 || self.pixel == self.x || self.pixel == self.x + 1 {
                self.crt_buffer.push('#');
            } else {
                self.crt_buffer.push('.');
            }

            self.pixel += 1;

            // Reset pixel position after line limit of 40
            if self.pixel == 40 {
                self.pixel = 0;
            }

            self.x += op.parse::<isize>().unwrap_or(0);
            self.ops += 1;

            if self.ops == 20 {
                self.signal_strength.push(self.x * 20);
            } else if self.ops == 60 {
                self.signal_strength.push(self.x * 60);
            } else if self.ops == 100 {
                self.signal_strength.push(self.x * 100);
            } else if self.ops == 140 {
                self.signal_strength.push(self.x * 140);
            } else if self.ops == 180 {
                self.signal_strength.push(self.x * 180);
            } else if self.ops == 220 {
                self.signal_strength.push(self.x * 220);
            }
        }
    }

    pub fn do_part(&self) {
        println!("Part One:");
        println!("    The signal strength is {}", self.signal_strength.iter().sum::<isize>());
        println!("Part Two:");
        println!("{}", self);
    }
}

impl fmt::Display for Processor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "    CRT Display:").expect("Should be able to write CRT Display.");

        for (idx, _) in self.crt_buffer.iter().enumerate() {
            if idx % 40 == 0 {
                write!(f, "\n    ").expect("Should be able to write a new line.");
            }

            write!(f, "{}", self.crt_buffer[idx]).expect("Should be able to print output.");
        }

        writeln!(f)
    }
}

impl Default for Processor {
    fn default() -> Self {
        Processor {
            x: 1,
            ops: 0,
            signal_strength: Vec::<isize>::new(),
            crt_buffer: Vec::<char>::with_capacity(240),
            pixel: 0,
        }
    }
}
