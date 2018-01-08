use std::collections::HashMap;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::str::FromStr;
use std::env::args;
use std::process::exit;

enum Operator {
    EQ, // ==
    NE, // !=
    LT, // <
    GT, // >
    GE, // >=
    LE, // <=
}

impl Operator {
    fn from_str(s: &str) -> Option<Operator> {
        match s {
            "==" => Some(Operator::EQ),
            "!=" => Some(Operator::NE),
            "<" => Some(Operator::LT),
            ">" => Some(Operator::GT),
            ">=" => Some(Operator::GE),
            "<=" => Some(Operator::LE),
            _ => None,
        }
    }

    fn eval<T>(&self, a: &T, b: &T) -> bool 
    where
        T: PartialEq + PartialOrd,
    {
        match self {
            &Operator::EQ => a.eq(b),
            &Operator::NE => a.ne(b),
            &Operator::LT => a.lt(b),
            &Operator::GT => a.gt(b),
            &Operator::GE => a.ge(b),
            &Operator::LE => a.le(b),
        }
    }
}

enum Opcode {
    Increment,
    Decrement,
}

impl Opcode {
    fn from_str(s: &str) -> Option<Opcode> {
        match s {
            "inc" => Some(Opcode::Increment),
            "dec" => Some(Opcode::Decrement),
            _ => None,
        }
    }

    fn apply<T>(&self, reg: &mut T, val: T)
    where
        T: AddAssign + SubAssign,
    {
        match self {
            &Opcode::Increment => reg.add_assign(val),
            &Opcode::Decrement => reg.sub_assign(val),
        };
    }
}

struct Instruction<T>
where
    T: PartialEq + PartialOrd + AddAssign + SubAssign 
{
    register: String,
    op: Opcode,
    operand: T,
    condition_register: String,
    condition: Operator,
    condition_operand: T,
}

impl<T> Instruction<T>
where
    T: PartialEq + PartialOrd + AddAssign + SubAssign + FromStr
{
    fn from_line(line: &str) -> Option<Instruction<T>> {

        let symbols = line.split_whitespace().collect::<Vec<&str>>();

        if symbols.len() != 7 {
            return None;
        }

        let op = Opcode::from_str(symbols[1]);
        let operand = symbols[2].parse::<T>();
        let cond_op = Operator::from_str(symbols[5]);
        let cond_operand = symbols[6].parse::<T>();

        if let (Some(op), Ok(operand), Some(cond_op), Ok(cond_operand)) = (op, operand, cond_op, cond_operand) {
            Some(Instruction {
                register: String::from(symbols[0]),
                op: op,
                operand: operand,
                condition_register: String::from(symbols[4]),
                condition: cond_op,
                condition_operand: cond_operand
            })
        }
        else {
            None
        }
    }
}

fn register_dump(input: String) -> HashMap<String, i32> {
    
    let mut registers: HashMap<String,i32> = HashMap::new();

    for line in input.lines() {
        if let Some(mut instruction) = Instruction::<i32>::from_line(&line) {

            let condition_reg_val: i32 = *registers.entry(instruction.condition_register).or_insert(0);
            let condition_operand = instruction.condition_operand;

            if instruction.condition.eval(&condition_reg_val, &condition_operand) {

                let reg_val = registers.entry(instruction.register).or_insert(0);
                instruction.op.apply(reg_val, instruction.operand);
            }
        }
    }

    registers
}

fn print_usage() {
    println!("Day 8: I Heard You Like Registers");
    println!("Usage:");
    println!("08 <input>");
    println!("  <input> - Input tree (see AoC example).");
    println!("            Typically from a file (e.g. \"$(cat inputfile)\"");
}

fn main() {
    if let (2, Some(input)) = (args().count(), args().nth(1)) {
        println!("{}", register_dump(input).values().max().unwrap());
    } else {
        print_usage();
        exit(-1);
    }
}


#[cfg(test)]
mod test {
    use super::*;
}
