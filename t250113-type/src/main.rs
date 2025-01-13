// 定义货币金额的类型别名，以分为单位
type CurrencyAmount = i64;
// 定义利率的类型别名，精确到小数点后四位
type InterestRate = f64;

// 计算利息的函数
fn calculate_interest(principal: CurrencyAmount, rate: InterestRate) -> CurrencyAmount {
    // 将金额转换为元，计算利息，再转换回分
    let principal_in_dollars = principal as f64 / 100.0;
    let interest_in_dollars = principal_in_dollars * rate;
    (interest_in_dollars * 100.0) as i64
}

fn main() {
    let principal: CurrencyAmount = 100000; // 1000元，以分为单位
    let rate: InterestRate = 0.05; // 5% 的利率
    let interest = calculate_interest(principal, rate);
    println!("计算出的利息为: {} 分", interest);
}
