use std::str::FromStr;
use bigdecimal::BigDecimal;

// 逆ポーランド表記の文字列を受け取り、結果を返す
pub fn calcurate(exp: &str) -> BigDecimal {
    let mut stack = Vec::<BigDecimal>::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = BigDecimal::from_str(token) {
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

fn execute_operator<F>(stack: &mut Vec<BigDecimal>, operation: F)
where 
    F: Fn(BigDecimal, BigDecimal) -> BigDecimal,
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
        assert_eq!("10.19", format!("{}", ans));
    }

    #[test]
    fn subtraction(){
        let ans = calcurate("1.21 8.98 - ");
        assert_eq!("-7.77", format!("{}", ans));
    }

    #[test]
    fn multiplication(){
        let ans = calcurate("1.21 8.98 * ");
        assert_eq!("10.8658", format!("{}", ans));
    }

    #[test]
    fn division(){
        let ans = calcurate("0.24 0.12 / ");
        assert_eq!("2", format!("{}", ans));
    }

    #[test]
    fn three_operands(){
        let ans = calcurate("1.21 8.98 0.74 + - ");
        assert_eq!("-8.51", format!("{}", ans));
    }
}