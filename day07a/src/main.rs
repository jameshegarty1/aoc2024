use::atoi::atoi;

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
    Multiply
}

fn get_ith_operators(i: &usize, n: &usize) -> Vec<Operator> {
    let mut operators = Vec::with_capacity(*n);
    //bitwise manipulations
    //lets say Add = 0 and Multiply = 1
    for position in 0..*n {
        let is_multiply = (i & (1 << position)) != 0;

        operators.push(if is_multiply {
            Operator::Multiply
        } else {
            Operator::Add
        });
    }

    operators
}

fn main() {
    let equations: Vec<Equation> = include_bytes!("../input")
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Equation::from_bytes(line))
        .collect();


    //Loop over equations
    //Number of operations per equation n = numbers.len()-1
    //Number of combinations = 2 ^ n
    //How to construct the combinations?
    //

    let mut overall_solution = 0;
    
    for equation in equations.iter() {
        let n_operators = equation.numbers.len() - 1;
        let n_combinations = 1 << n_operators;  // 2^n_operators


        let mut i = 0;
        while i < n_combinations {
            let operators = get_ith_operators(&i,&n_operators);
            let mut solution: i64 = equation.numbers[0].clone();
            for (j, operator) in operators.iter().enumerate() {
                match operator {
                    Operator::Add => solution += equation.numbers[j+1],
                    Operator::Multiply => solution *= equation.numbers[j+1]
                }
            }
            i += 1;

            if solution == equation.target {
                overall_solution += solution;
                break;
            }
            
        }
    }


    println!("{}", overall_solution);
        
}
