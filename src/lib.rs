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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition(){
        let ans = calcurate("1.21 8.98 + ");
        assert_eq!("10.19", format!("{:.2}", ans));
    }

    #[test]
    fn subtraction(){
        let ans = calcurate("1.21 8.98 - ");
        assert_eq!("-7.77", format!("{:.2}", ans));
    }

    #[test]
    fn multiplication(){
        let ans = calcurate("1.21 8.98 * ");
        assert_eq!("10.87", format!("{:.2}", ans));
    }

    #[test]
    fn division(){
        let ans = calcurate("1.21 8.98 / ");
        assert_eq!("0.13", format!("{:.2}", ans));
    }

    #[test]
    fn three_operands(){
        let ans = calcurate("1.21 8.98 0.74 + - ");
        assert_eq!("-8.51", format!("{:.2}", ans));
    }
}