use std::io;
use rand::Rng;

/// 经典的石头剪刀布游戏, 玩家和电脑进行游戏
fn main() {
    println!("游戏开始了");
    loop {
        round();
        println!("进行下一轮");
    }
}

/// 石头 剪刀 布 的枚举类型
#[derive(Debug)]
enum Pose {
    /// 石头
    Rock,
    /// 布
    Paper,
    /// 剪刀
    Scissors
}

impl Pose {
    /// 获取随机生成的电脑出拳的结果
    fn get(id: u32) -> Self{
        match id {
            0 => Pose::Rock,
            1 => Pose::Paper,
            2 => Pose::Scissors,
            _ => Pose::Paper
        }
    }
}

/// 比较电脑和玩家的出拳类型,确定输赢
fn compare(computer: Pose, player:Pose) -> () {
    match (computer, player) {
        (Pose::Rock, Pose::Rock) | (Pose::Paper, Pose::Paper) | (Pose::Scissors, Pose::Scissors) => println!("平局"),
        (Pose::Rock, Pose::Paper) | (Pose::Paper, Pose::Scissors) | (Pose::Scissors, Pose::Rock) => println!("你赢了"),
        (Pose::Rock, Pose::Scissors) | (Pose::Paper, Pose::Rock) | (Pose::Scissors, Pose::Paper) => println!("你输了"),
    }
}

/// 单轮游戏从这里开始
fn round() {
    let user_pose: Pose;
    let rand_num = rand::thread_rng().gen_range(0..3) as u32;
    let computer_pose = Pose::get(rand_num);
    loop{
        let input = read_use_pose();
        match input {
            Ok(input) => {user_pose = input;break;},
            Err(input)  => {println!("{}", input); continue;}
        }
    }
    println!("电脑: {:?}, 你: {:?}", computer_pose, &user_pose);
    compare(computer_pose, user_pose);
}

/// 用户输入
fn read_use_pose() -> Result<Pose, &'static str> {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).map_err(|_| "Fail to input")?;
    match input_string.trim().to_uppercase().as_str() {
        "R" => Ok(Pose::Rock),
        "P" => Ok(Pose::Paper),
        "S" => Ok(Pose::Scissors),
        _ => Err("请输入有效的值(R, P, S)")
    }
}