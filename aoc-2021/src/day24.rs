use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug)]
pub enum Operand {
    Register(usize),
    Literal(isize),
}

#[derive(Copy, Clone, Debug)]
pub enum Inst {
    Inp(usize),
    Add(usize, Operand),
    Mul(usize, Operand),
    Div(usize, Operand),
    Mod(usize, Operand),
    Eql(usize, Operand),
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Inst> {
    input
        .lines()
        .map(|line| {
            let (instruction, rest) = line.split_once(" ").unwrap();
            
            if instruction == "inp" {
                let rega = match rest {
                    "w" => 0,
                    "x" => 1,
                    "y" => 2,
                    "z" => 3,
                    _ => panic!(),
                };
                Inst::Inp(rega)
            }
            else {
                let (rega, regb) = rest.split_once(" ").unwrap();

                let rega = match rega {
                    "w" => 0,
                    "x" => 1,
                    "y" => 2,
                    "z" => 3,
                    _ => panic!(),
                };

                let regb = match regb {
                    "w" => Operand::Register(0),
                    "x" => Operand::Register(1),
                    "y" => Operand::Register(2),
                    "z" => Operand::Register(3),
                    _ => Operand::Literal(regb.trim().parse().unwrap()),
                };

                match instruction {
                    "add" => Inst::Add(rega, regb),
                    "mul" => Inst::Mul(rega, regb),
                    "div" => Inst::Add(rega, regb),
                    "mod" => Inst::Mul(rega, regb),
                    "eql" => Inst::Add(rega, regb),
                    _ => panic!(),
                }
            }
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn solve_part1(program: &[Inst]) -> isize {
    let mut digits = [9; 14];

    let mut iters = 0;

    loop {
        let regs = eval_faster(&digits);

        if regs[13] == 0 {
            break;
        }

        for i in 0..14 {
            let digit = 14 - 1 - i;

            if digits[digit] == 1 {
                digits[digit] = 9;
            }
            else {
                digits[digit] -= 1;
                break;
            }
        }

        iters += 1;

        if iters % 10_000_000 == 0 {

            //let regs_standard = evaluate(program, &digits);
            let regs_fast = eval_fast(&digits);

            let history_faster = eval_faster(&digits);

            println!("Checking {} z: {:?}", digits.iter().fold(0, |sum, digit| sum * 10 + digit), history_faster);

            assert_eq!(regs_fast[3], history_faster[13]);

        }
    }

    digits.iter().fold(0, |sum, digit| sum * 10 + digit)
}

pub fn read(registers: &[isize], reg: Operand) -> isize {
    match reg {
        Operand::Literal(lit) => lit,
        Operand::Register(reg) => registers[reg],
    }
}

pub fn eval_fast(inputs: &[isize]) -> [isize; 4] {
    let mut input_index = 0;

    let mut w = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    w = inputs[input_index];
    input_index += 1;
    y = w + 6;
    z = w + 6;

    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) + 11;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 12) * x;
    z = z + y;

    // line 24
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) + 10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 5) * x;
    z = z + y;

    // line 42
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) + 10;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 10) * x;
    z = z + y;

    // line 60
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) - 16;
    z = z / 26;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 7) * x;
    z = z + y;

    // line 78
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) + 14;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = w * x;
    z = z + y;

    // line 96
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) + 12;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 4) * x;
    z = z + y;

    // line 114
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) - 4;
    z = z / 26;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 12) * x;
    z = z + y;

    // line 132
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) + 15;
    z = z / 1;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 14) * x;
    z = z + y;

    // line 150
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) -7;
    z = z / 26;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 13) * x;
    z = z + y;

    // line 168
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) - 8;
    z = z / 26;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 10) * x;
    z = z + y;

    // line 186
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) - 4;
    z = z / 26;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 11) * x;
    z = z + y;

    // line 204
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) - 15;
    z = z / 26;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 9) * x;
    z = z + y;

    // line 222
    w = inputs[input_index];
    input_index += 1;
    x = (z % 26) - 8;
    z = z / 26;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y = 25 * x + 1;
    z = z * y;
    y = (w + 9) * x;
    z = z + y;

    [w, x, y, z]
}

pub fn eval_faster(inputs: &[isize]) -> [isize; 14] {
    let mut w;
    let mut x;
    let mut z;
    
    let mut output = [0; 14];

    w = inputs[0];
    z = w + 6;
    output[0] = z;
    
    // line 6
    w = inputs[1];
    if (z % 26) + 11 != w {
        z = z * 26;
        z += w + 12;
    }
    output[1] = z;
    
    // line 24
    w = inputs[2];
    if (z % 26) + 10 != w {
        z = z * 26;
        z += w + 5;
    }
    output[2] = z;
    
    // line 42
    w = inputs[3];
    if (z % 26) + 10 != w {
        z = z * 26;
        z += w + 10;
    }
    output[3] = z;
    
    // line 60
    w = inputs[4];
    x = (z % 26) - 16;
    z = z / 26;
    if x != w {
        z = z * 26;
        z += w + 7;
    }
    output[4] = z;
    
    // line 78
    w = inputs[5];
    if (z % 26) + 14 != w {
        z = z * 26;
        z += w;
    }
    output[5] = z;
    
    // line 96
    w = inputs[6];
    x = (z % 26) + 12;
    z = z / 26;
    if x != w {
        z = z * 26;
        z += w + 4;
    }
    output[6] = z;
    
    // line 114
    w = inputs[7];
    x = (z % 26) - 4;
    z = z / 26;
    if x != w {
        z = z * 26;
        z += w + 12;
    }
    output[7] = z;
    
    // line 132
    w = inputs[8];
    if (z % 26) + 15 != w {
        z = z * 26;
        z += w + 14;
    }
    output[8] = z;
    
    // line 150
    w = inputs[9];
    x = (z % 26) - 7;
    z = z / 26;
    if x != w {
        z = z * 26;
        z += w + 13;
    }
    output[9] = z;
    
    // line 168
    w = inputs[10];
    x = (z % 26) - 8;
    z = z / 26;
    if x != w {
        z = z * 26;
        z += w + 10;
    }
    output[10] = z;
    
    // line 186
    w = inputs[11];
    x = (z % 26) - 4;
    z = z / 26;
    if x != w {
        z = z * 26;
        z += w + 11;
    }
    output[11] = z;
    
    // line 204
    w = inputs[12];
    x = (z % 26) - 15;
    z = z / 26;
    if x != w {
        z = z * 26;
        z += w + 9;
    }
    output[12] = z;
    
    // line 222
    w = inputs[13];
    x = (z % 26) - 8;
    z = z / 26;
    if x != w {
        z = z * 26;
        z += w + 9;
    }
    output[13] = z;

    output
}

#[test]
pub fn test_guess_inputs() {
    // Offsets are on lines 11 + 18*n
    let offset = [11, 10, 10, -16, 14, 12, -4, 15, -7, -8, -4, -15, -8 ];
    // Divisors are on lines 10 + 18*n
    let divisors = [1, 1, 1, 26, 1, 1, 26, 1, 26, 26, 26, 26, 26];
    // Adders are on lines 21 + 18*n
    let adders = [12, 5, 10, 7, 0, 4, 12, 14, 13, 10, 11, 9, 9];

    let guess = [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 0];
    let history = eval_faster(&guess);
    println!("(input, low growth input, z) = {:?}", history.iter().zip(offset.iter().zip(guess.iter())).map(|(z, (offset, guess))| (*guess, z % 26 + offset, *z)).collect::<Vec<(isize, isize, isize)>>());

    let guess = [9, 9, 9, 9, 3, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    let history = eval_faster(&guess);
    println!("(input, low growth input, z) = {:?}", history.iter().zip(offset.iter().zip(guess.iter())).map(|(z, (offset, guess))| (*guess, z % 26 + offset, *z)).collect::<Vec<(isize, isize, isize)>>());

    let guess = [9, 9, 9, 9, 3, 1, 9, 9, 9, 9, 9, 9, 5, 9];
    let history = eval_faster(&guess);
    println!("(input, low growth input, z) = {:?}", history.iter().zip(offset.iter().zip(guess.iter())).map(|(z, (offset, guess))| (*guess, z % 26 + offset, *z)).collect::<Vec<(isize, isize, isize)>>());


}



pub fn evaluate(program: &[Inst], inputs: &[isize]) -> [isize;4] {
    let mut registers = [0isize; 4];
    let mut input_index = 0;

    for &inst in program {
        match inst {
            Inst::Inp(a) => {
                registers[a] = inputs[input_index];
                input_index += 1;
            },
            Inst::Add(a, b) => {
                registers[a] = registers[a] + read(&registers, b);
            },
            Inst::Mul(a, b) => {
                registers[a] = registers[a] * read(&registers, b);
            },
            Inst::Div(a, b) => {
                registers[a] = registers[a] / read(&registers, b);
            },
            Inst::Mod(a, b) => {
                registers[a] = registers[a] % read(&registers, b);
            },
            Inst::Eql(a, b) => {
                registers[a] = if registers[a] == read(&registers, b) { 1 } else { 0 };
            },
        }
    }

    registers
}