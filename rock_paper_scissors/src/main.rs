use std::io;
use rand::Rng;

fn main() {
    println!("游戏开始了");
    loop {
        round();
        println!("进行下一轮");
    }
}

#[derive(Debug)]
enum Pose {
    Rock,
    Paper,
    Scissors
}

impl Pose {
    fn get(id: u32) -> Self{
        match id {
            0 => Pose::Rock,
            1 => Pose::Paper,
            2 => Pose::Scissors,
            _ => Pose::Paper
        }
    }
}


fn compare(computer: Pose, player:Pose) -> () {
    match (computer, player) {
        (Pose::Rock, Pose::Rock) | (Pose::Paper, Pose::Paper) | (Pose::Scissors, Pose::Scissors) => println!("平局"),
        (Pose::Rock, Pose::Paper) | (Pose::Paper, Pose::Scissors) | (Pose::Scissors, Pose::Rock) => println!("你赢了"),
        (Pose::Rock, Pose::Scissors) | (Pose::Paper, Pose::Rock) | (Pose::Scissors, Pose::Paper) => println!("你输了"),
    }
}


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