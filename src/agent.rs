pub mod locate {

    use reqwest;
    use serde_json::Value;
    use std::error::Error;
    use crate::Loc;

    pub fn locate(input: &str) -> Result<Loc, Box<dyn Error>> {
        
        let key = "<API-Key>";
        
        let address = if input.contains("北京大学") {
            input
        }
        else {
            &format!("北京大学{}", input)
        };

        let url = format!(
            "https://restapi.amap.com/v3/geocode/geo?key={}&address={}&city=北京",
            key, 
            address,
        );

        let resp = reqwest::blocking::get(&url)?.json::<Value>()?;

        if let Some(geocodes) = resp["geocodes"].as_array() {
            if let Some(result) = geocodes.get(0) {

                let loc: Vec<&str> = result["location"].as_str().ok_or("")?.split(',').collect();

                let longitude: f64 = loc[0].parse()?;
                let latitude: f64 = loc[1].parse()?;

                let x: usize = (longitude * 87873.931624 - 10220153.0) as usize;
                let y: usize = (-latitude * 114219.1142 + 4567964.9) as usize;
                return Ok((x, y));
            }
        }
        Err("".into())
    }
}


pub mod chat {

    use std::error::Error;
    use reqwest;
    use reqwest::blocking::Client;  
    use serde::{Deserialize, Serialize};
    use std::io::{self, Write};

        #[derive(Serialize, Deserialize)]
    struct Message {
        role: String,
        content: String,
    }

    #[derive(Deserialize)]
    struct ApiResponse {
        choices: Vec<Choice>,
    }

    #[derive(Deserialize)]
    struct Choice {
        message: Message,
    }

    #[derive(Deserialize)]
    pub struct PathInfo {
        pub from: String,
        pub to: String,
    }

    #[derive(Deserialize)]
    pub struct CycleInfo {
        pub loc: String,
    }

    #[derive(Deserialize)]
    pub enum Resp {
        Path(PathInfo),
        Cycle(CycleInfo),
        Bye(String),
        Answer(String),
    }

    pub struct ChatAgent {
        history: String,
    }

    impl ChatAgent {
        pub fn new() -> Self {
            Self {
                history: String::from("")
            }
        }

        fn add_user_msg(&mut self, s: &str) {
            self.history = format!(r#"{}, {{"role": "user", "content": "{}" }}"#, self.history, s);
        }

        fn add_assistant_msg(&mut self, s: &str) {
            self.history = format!(r#"{}, {{"role": "assistant", "content": "{}" }} "#, self.history, s);
        }

        pub fn get_resp(&mut self) -> Result<Resp, Box<dyn Error>> {

            let client = Client::new();

            print!("[User]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            self.add_user_msg(&input[..input.len() - 2]);

            let prompt = "你的身份是北大导览小助手，名字叫PKU-Guide，你的答复要生动活泼，但不要使用颜文字和表情。\
            你将获得对话历史，请根据此把最新的一句用户请求解析为json格式，json里面只有一项，关键字待定。\
            情况一，即用户必须明确表明要退出或不需进行更多请求，关键字为Bye，内容为你写的答复；\
            情况二，即用户普通的交流（可能是感谢或询问等，询问可能是地点，但非询问路线），关键字为Answer，内容为你的答复；\
            情况三，即用户必须明确表明需要路线相关请求，关键字为Path，你需要在里面放一个json，关键字为from和to，它们内容可以为三种情况：\
            1、地名（如北京大学这样具体的地名，有可能是用户所在地出发且用户曾经提及所在地）；\
            2、“user”（如“我的位置”“这里”“我要去”等等或表明从用户处出发的含义，且用户未曾提及其位置，如果能够推断出用户位置，则属于前一种情况）；\
            3、“map”（如“一个地方”“某个点位”“地图选择”等等）（均不含引号）\
            情况四，即用户必须明确表明需要一条游览路线，关键字为Cycle，你需要在里面放一个json，关键字为loc，它们的情况的情况三的三种情况相同。\
            请注意你此时的回答任何情况都是json格式";

            let init = "你好，我是PKU-Guide小助手。我可以帮你解答任何问题，也可以为你规划路线~";

            let body = format!(
                r#"{{"model": "deepseek-chat", "messages": [{{"role": "system", "content": "{}"}},{{"role": "assistant", "content": "{}"}}{}]}}"#, 
                &prompt, &init, &self.history
            );

            let response = client
                .post("https://api.deepseek.com/chat/completions")
                .header("Content-Type", "application/json")
                .header("Authorization", "Bearer <API-Key>") 
                .body(body)
                .send()?;  

            if response.status().is_success() {
                let response: ApiResponse = response.json()?;  
                let content = &response.choices[0].message.content;
                self.add_assistant_msg(&content.replace('"', r#"\""#).replace('\n', ""));
                let resp = serde_json::from_str(&content.replace("json", "").replace("`", "").replace('\n', ""))?;
                return Ok(resp);
            }
            Err("".into())
        }
    }

}
