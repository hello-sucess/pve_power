extern crate serde;

use std::{env, io};
use std::str::FromStr;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use confy;
use serde::{Serialize, Deserialize};
use std::string::String;


#[derive(Debug,Serialize, Deserialize)]
struct Config{
    enable: bool,
    sleep: i32,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {enable: true,
              sleep: 0,
        }
    }
}

fn pve_list()->Vec<String> {
    let output = Command::new("sh").arg("-c").arg("qm list|awk 'NR == 1 {next} {print $1}'").output().expect("GET PVE LIST ERROR!");
    let id_list = String::from_utf8(output.stdout).unwrap();
    let id_list = id_list.split('\n');
    let output = Command::new("sh").arg("-c").arg("qm list|awk 'NR == 1 {next} {print $3}'").output().expect("GET PVE LIST ERROR!");
    let state_list = String::from_utf8(output.stdout).unwrap();
    let state_list = state_list.split('\n');
    let list= id_list.zip(state_list);
    let mut result_vec:Vec<String> = Vec::new();

    for i in list{
        if i.1 != "stopped" && i.0 != ""{
            result_vec.push(String::from(i.0));
        }
    }
    return result_vec;
}


fn main()->Result<(),confy::ConfyError>{
    //获取命令行参数
    let args: Vec<String> = env::args().collect();
    //载入配置文件
    let mut conf:Config = confy::load_path("/etc/conf.config")?;

    if args.len() >1{
        println!("{}","修改配置!");
    //修改配置
        if args[1] == "start" {
            conf.enable = true;
        }else if args[1] == "stop" {
            conf.enable = false;
        }else {
            let day = args[1].parse::<i32>().unwrap();
            conf.sleep = day;
        }

    }else if conf.sleep != 0{
        println!("{}","休眠中...!");
        //休眠天数减去1
        conf.sleep -=1;
    }else if conf.enable{
        println!("检查正在执行的虚拟机!");
        //开始检查开机虚拟机、并执行挂起
        let list = pve_list();
        for i in list{
           //如果状态不是运行中则执行挂起
            let output = Command::new("sh").arg("-c").arg(format!("qm suspend {} --todisk",i)).output().expect("pve to disk error!");
            let result = String::from_utf8(output.stdout).unwrap();
            sleep(Duration::from_secs(100));

        }
        //关机!
        println!("关机中......");
        let output = Command::new("shutdown").arg("now").output().expect("shutdown error");
    }
    //输出当前配置
    println!("enable={}\nSleep={}",conf.enable,conf.sleep);
    confy::store_path("/etc/conf.config",conf)?;
    Ok(())
}
