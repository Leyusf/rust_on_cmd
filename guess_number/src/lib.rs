use rand::Rng;
use std::io;
/// 随机数发牌员
pub struct Dealer{
    number: u32,
}
/// 猜测结果状态
pub enum State {
    /// 猜对
    Catch,
    /// 猜测值偏小
    Under,
    /// 猜测值偏大
    Over
}

impl Dealer {
    /// 新建随机数发牌员
    pub fn new() -> Self {
        let number = rand::thread_rng().gen_range(1..101);
        Dealer{
            number
        }
    }

    /// 检测是否猜中
    pub fn check(&self, guess_number: u32) -> State{
        match self.number == guess_number {
            true => State::Catch,
            false => match self.number < guess_number {
                true => State::Over,
                _ => State::Under
            }
        }
    }
}

/// 开始一轮游戏
pub fn round() {
    let mut user_guess: u32;
    let mut attempts = 0;
    let dealer = Dealer::new();
    loop {
        let input = read_input();
        match input {
            Ok(input) => user_guess = input,
            _ => {println!("{}", input.unwrap());continue}
        };
        attempts += 1;
        match dealer.check(user_guess) {
            State::Catch => {println!("你已经成猜到了数字，总共猜了{}次", attempts); return},
            State::Over => println!("偏大了，你已经猜了{}次", attempts),
            State::Under => println!("偏小了，你已经猜了{}次", attempts),
        }
    }
}

/// 辅助函数，用于读取用户输入并返回 u32 类型的数字
pub fn read_input() -> Result<u32, &'static str> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).map_err(|_| "Failed to read line")?;
    match input_string.trim().parse() {
        Ok(num) => Ok(num),
        Err(_) => Err("Please input a valid number"),
    }
}

