use reqwest::header::HeaderMap;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Deserialize, Serialize)]
struct Dingtalk {
    msgtype: String,
    text: DingtalkContent,
}

#[derive(Hash, Eq, PartialEq, Debug, Deserialize, Serialize)]
struct DingtalkContent {
    content: String,
}

pub async fn send(message: String) -> i32 {
    println!("POST请求发送钉钉消息!");

    // let url="https://oapi.dingtalk.com/robot/send?access_token=9c7d42da895c54e6b5f5701a6689684fb4160102e2fc493e895dc081b80f0fd9";  //这个是预警群
    let url="https://oapi.dingtalk.com/robot/send?access_token=b3f029cdb1e3f2c8f9231b5e1641d1d69d33125e108fc3b29ab968f6a36d8b38"; //这个是爆块群
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("ontent-Type", "application/json".parse().unwrap());

    // let mut data = HashMap::new();
    // data.insert("msgtype", "text");
    // let mut dingtalk = Dingtalk::new("kdasdf");
    // data.insert("text", dingtalk);
    let message = format!("MASS矿机通知：{}", message);

    let content = DingtalkContent { content: message };
    let data = Dingtalk {
        msgtype: "text".to_string(),
        text: content,
    };

    let response = client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap();

    let temp_json = response.json::<HashMap<String, Value>>().await.unwrap();
    println!("{:#?}", temp_json);
    println!("======================9=====================");
    let kasdf = 89;
    kasdf
}


pub async fn send_xfx(message: String) -> i32 {
    println!("POST请求发送钉钉消息!");

    // let url="https://oapi.dingtalk.com/robot/send?access_token=9c7d42da895c54e6b5f5701a6689684fb4160102e2fc493e895dc081b80f0fd9";  //这个是预警群
    let url="https://oapi.dingtalk.com/robot/send?access_token=6105782b73d84dc9ac231da59c9340ec34df266a33f8671b080888c5293baa62"; //这个是爆块群
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("ontent-Type", "application/json".parse().unwrap());

    // let mut data = HashMap::new();
    // data.insert("msgtype", "text");
    // let mut dingtalk = Dingtalk::new("kdasdf");
    // data.insert("text", dingtalk);
    let message = format!("XFX矿机通知：{}", message);

    let content = DingtalkContent { content: message };
    let data = Dingtalk {
        msgtype: "text".to_string(),
        text: content,
    };

    let response = client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap();

    let temp_json = response.json::<HashMap<String, Value>>().await.unwrap();
    println!("{:#?}", temp_json);
    println!("======================9=====================");
    let kasdf = 89;
    kasdf
}


pub async fn send_xch(message: String) -> i32 {
    println!("POST请求发送钉钉消息!");

    // let url="https://oapi.dingtalk.com/robot/send?access_token=9c7d42da895c54e6b5f5701a6689684fb4160102e2fc493e895dc081b80f0fd9";  //这个是预警群
    let url="https://oapi.dingtalk.com/robot/send?access_token=8ab7bcaf160e74f02b4cf14c7506c39fef80bf37bbe7edb508e01811cce9e42d"; //这个是爆块群
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("ontent-Type", "application/json".parse().unwrap());

    // let mut data = HashMap::new();
    // data.insert("msgtype", "text");
    // let mut dingtalk = Dingtalk::new("kdasdf");
    // data.insert("text", dingtalk);
    let message = format!("XCH爆块通知：{}", message);

    let content = DingtalkContent { content: message };
    let data = Dingtalk {
        msgtype: "text".to_string(),
        text: content,
    };

    let response = client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap();

    let temp_json = response.json::<HashMap<String, Value>>().await.unwrap();
    println!("{:#?}", temp_json);
    println!("======================9=====================");
    let kasdf = 89;
    kasdf
}


pub async fn send_cru(message: String) -> i32 {
    println!("POST请求发送钉钉消息!");

    // let url="https://oapi.dingtalk.com/robot/send?access_token=9c7d42da895c54e6b5f5701a6689684fb4160102e2fc493e895dc081b80f0fd9";  //这个是预警群
    let url="https://oapi.dingtalk.com/robot/send?access_token=d998cf3dadbac1402dcbff2fc5d6c9ac72c7f814e75871c0a18b76b5fc70ad8c"; //这个是爆块群
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("ontent-Type", "application/json".parse().unwrap());

    // let mut data = HashMap::new();
    // data.insert("msgtype", "text");
    // let mut dingtalk = Dingtalk::new("kdasdf");
    // data.insert("text", dingtalk);
    let message = format!("CRU收益通知：{}", message);

    let content = DingtalkContent { content: message };
    let data = Dingtalk {
        msgtype: "text".to_string(),
        text: content,
    };

    let response = client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap();

    let temp_json = response.json::<HashMap<String, Value>>().await.unwrap();
    println!("{:#?}", temp_json);
    println!("======================9=====================");
    let kasdf = 89;
    kasdf
}
