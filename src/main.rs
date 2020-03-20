// 逆ポーランド記法で計算するプログラム
fn main() {
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let ans = calcurate(exp);

    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    println!("{} = {:.4}", exp, ans);
}

fn calcurate(exp: &str) -> f64 {
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
