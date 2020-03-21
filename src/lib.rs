// 逆ポーランド表記の文字列を受け取り、結果を返す
pub fn calcurate(exp: &str) -> f64 {
    let mut stack = Vec::<f64>::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            match token {
                "+" => execute_operator(&mut stack, |x, y| x + y),
                "-" => execute_operator(&mut stack, |x, y| x - y),
                "*" => execute_operator(&mut stack, |x, y| x * y),
                "/" => execute_operator(&mut stack, |x, y| x / y),
                _ => panic!("Unkown operator:{}", token),
            }
        }
    }

    // 計算終了後、結果を取り出して返す
    return stack.pop().expect("Stack underflow");
}

fn execute_operator<F>(stack: &mut Vec<f64>, operation: F)
where 
    F: Fn(f64, f64) -> f64,
{
    let y = stack.pop().expect("Stack underflow!");
    let x = stack.pop().expect("Stack underflow!");
    let z = operation(x, y);
    stack.push(z);
}