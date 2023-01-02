const INTERESTING_CYCLES: &[u32] = &[20, 60, 100, 140, 180, 220];
const HEIGHT: usize = 6;
const WIDTH: usize = 40;

pub fn input() -> &'static str {
    include_str!("../../input/day10.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day10.txt")
}

pub fn solve(input: &str) -> (usize, String) {
    let mut cpu = CPU::new();
    let instructions = input.lines().map(Instruction::from_str);
    cpu.process(instructions);
    let converted = common::ocr::convert(&cpu.readout());
    (
        cpu.signal_strength as usize,
        if converted == "" {
            cpu.readout()
        } else {
            converted
        },
    )
}

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        match s.split_once(' ').unwrap_or(("noop", "")) {
            ("addx", n) => Self::Addx(n.parse::<i32>().unwrap()),
            ("noop", _) => Self::Noop,
            (_, _) => unreachable!(),
        }
    }
}

struct CPU<'a> {
    register: i32,
    cycle: u32,
    signal_strength: i32,
    pixel: (usize, usize),
    crt: [[&'a str; WIDTH]; HEIGHT],
}

impl<'a> CPU<'a> {
    fn new() -> Self {
        Self {
            register: 1,
            cycle: 1,
            signal_strength: 0,
            pixel: (0, 0),
            crt: [["."; WIDTH]; HEIGHT],
        }
    }

    fn process<I>(&mut self, instructions: I)
    where
        I: IntoIterator<Item = Instruction>,
    {
        for instruction in instructions {
            match instruction {
                Instruction::Addx(n) => (0..2).for_each(|c| {
                    self.update_strength();
                    self.draw_pixel();
                    if c == 1 {
                        self.register += n;
                    }
                    self.cycle += 1;
                }),
                Instruction::Noop => {
                    self.update_strength();
                    self.draw_pixel();
                    self.cycle += 1;
                }
            }
        }
    }

    fn sprite_position(&mut self) -> [i32; 3] {
        [self.register - 1, self.register, self.register + 1]
    }

    fn update_strength(&mut self) {
        if INTERESTING_CYCLES.contains(&self.cycle) {
            self.signal_strength += self.cycle as i32 * self.register;
        }
    }

    fn draw_pixel(&mut self) {
        let (row, col) = self.pixel;
        if self.sprite_position().contains(&(col as i32)) {
            self.crt[row][col] = "#";
        }
        if col == 39 {
            self.pixel.1 = 0;
            self.pixel.0 += 1;
        } else {
            self.pixel.1 += 1;
        }
    }

    fn readout(&self) -> String {
        self.crt.map(|row| row.join("")).join("\n")
    }
}

common::test!(
    day10,
    (
        13140,
        String::from(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        )
    ),
    (13180, String::from("EZFCHJAB"))
);
