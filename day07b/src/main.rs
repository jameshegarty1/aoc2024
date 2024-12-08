use::atoi::atoi;
use std::collections::HashSet;

#[derive(Debug)]
struct Equation {
    target: i64,
    numbers: Vec<i64>
}

impl Equation{
    fn from_bytes(line: &[u8]) -> Self {
        let mut parts = line.split(|b| *b == b':');

        let target: i64 = atoi::<i64>(parts.next().unwrap()).unwrap(); 
        let numbers: Vec<i64> = parts
            .next()
            .unwrap()
            .split(|b| *b == b' ')
            .filter(|slice| !slice.is_empty())
            .map(|digits| atoi::<i64>(digits).unwrap())
            .collect();

        Equation { target, numbers }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
    Concat
}

fn get_ith_operators(i: &usize, n: &usize) -> Vec<Operator> {
    let mut operators = Vec::with_capacity(*n);
    //bitwise manipulations
    //lets say Add = 00 and Multiply = 01 and concat = 10 and 11 is unused.
    for position in 0..*n {
        let op_code = (i & (3 << (2 * position))) >> (2 * position);

        let op = match op_code {
            0 => Operator::Add,      // 00
            1 => Operator::Multiply, // 01
            2 => Operator::Concat,   // 10
            _ => continue           // Skip 11 case
        };
        operators.push(op);
    }

    operators
}

fn try_evaluate(numbers: &[i64], operators: &[Operator], target: i64) -> bool {
    let mut result = numbers[0];
    
    for (idx, op) in operators.iter().enumerate() {
        let next_num = numbers[idx + 1];
        
        match op {
            Operator::Add => {
                match result.checked_add(next_num) {
                    Some(sum) => result = sum,
                    None => return false
                }
            }
            Operator::Multiply => {
                match result.checked_mul(next_num) {
                    Some(product) => result = product,
                    None => return false
                }
            }
            Operator::Concat => {
                // Convert both numbers to strings
                let result_str = result.to_string();
                let next_str = next_num.to_string();
                
                // Check if concatenated result would be too long
                if result_str.len() + next_str.len() > 19 { // i64 max is 19 digits
                    return false;
                }
                
                // Try to parse the concatenated result
                match format!("{}{}", result, next_num).parse::<i64>() {
                    Ok(val) => result = val,
                    Err(_) => return false
                }
            }
        }
        
        // Early exit if we've exceeded target (except for concat which might make number smaller)
        if result > target && !matches!(op, Operator::Concat) {
            return false;
        }
    }
    
    result == target
}

fn main() {
    let equations: Vec<Equation> = include_bytes!("../input")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Equation::from_bytes(line))
        .collect();


    let mut solvable_equations = HashSet::new();
    
    for equation in equations.iter() {
        let n_operators = equation.numbers.len() - 1;
        let n_combinations = 1 << (2 * n_operators);

        for i in 0..n_combinations {
            let operators = get_ith_operators(&i,&n_operators);
            if operators.len() != n_operators {
                continue;
            }

            if try_evaluate(&equation.numbers, &operators, equation.target) {
                solvable_equations.insert(equation.target);
                break;
            }
            
        }
    }


    println!("{}", solvable_equations.iter().sum::<i64>());
        
}
