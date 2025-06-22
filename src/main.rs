use agent::chat::Resp;
use colored::Colorize;
use std::io::{self, Write};
use window::{fetch_loc, show_path};

type Loc = (usize, usize);

fn agent_print(s: &str) {
    let s = &format!("[PKU-Guide]: {}", s);
    println!("{}", s.green());
}

fn sys_print(s: &str) {
    let s = &format!("[System]: {}（请按回车继续）", s);
    print!("{}", s.red());
    io::stdout().flush().unwrap();
    read_enter();
}

fn read_enter() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn parse_loc(loc_name: &str, s: &str) -> Option<Loc> {
    match s {
        "user" => {
            sys_print("请选择您所在的位置");
            Some(fetch_loc())
        }
        "map" => {
            sys_print(&format!("请选择您{}的位置", loc_name));
            Some(fetch_loc())
        } 
        other => {
            let loc = agent::locate::locate(&other);
            match loc {
                Ok(loc) => {
                    if !matches!(loc.0, 1..window::WIDTH) || !matches!(loc.1, 1..window::HEIGHT) {
                        sys_print(&format!("{}不在地图范围内哦~", loc_name));
                        return None;
                    }
                    return Some(loc);  
                }
                _ => {
                    sys_print("地名解析失败，请重试");
                    return None;
                }
            }
        }
    }
}

fn main() {

    let mut chat_agent = agent::chat::ChatAgent::new();
    let map = map::Map::new();

    agent_print("你好，我是PKU-Guide小助手。我可以帮你解答任何问题，也可以为你规划路线~😊");    

    loop {
        let resp = chat_agent.get_resp();
        match resp {
            Ok(resp) => {
                match resp {
                    Resp::Answer(s) => {
                        agent_print(&s);
                    },
                    Resp::Bye(s) => {
                        agent_print(&s);
                        break;
                    }
                    Resp::Path(path) => {
                        if let Some(loc_s) = parse_loc("出发地", &path.from) {
                            if  let Some(loc_t) = parse_loc("目的地", &path.to) {
                                let v = map::path::get_path(&map, loc_s, loc_t);
                                sys_print("路线计算完成");
                                show_path(&v);
                            }
                        }
                    },
                    Resp::Cycle(cycle) => {
                        if let Some(loc) = parse_loc("出发地", &cycle.loc) {
                            let v = map::cycle::get_cycle(&map, loc);
                            sys_print("路线计算完成");
                            show_path(&v);    
                        }
                    },
                }
            },
            _ => {
                sys_print("请求解析失败，请重试");
            },
        }
    }

}

mod agent;

mod queue;

mod window;

mod map;