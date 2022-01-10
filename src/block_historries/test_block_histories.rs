pub fn test_file() {
    println!("测试文件读写");
    use std::fs::File;
    use std::io::BufReader;
    use std::io::BufWriter;
    use std::io::ErrorKind;
    use std::io::Read;
    use std::io::Write;
    use std::path::PathBuf;

    let cru_block_path = "cru_block.txt";
    let f = File::open(cru_block_path);
    let mut file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(cru_block_path) {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件{}失败", e),
            },
            other_error => panic!("打开文件出错:{:?}",),
        },
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("读文件到字符出错");

    println!("读到的文件：{:#?}", contents);

    let mut contents = contents.parse::<i64>().unwrap();
    contents += 1;
    println!("自增后的文件内容:{}", contents);
    let temp = contents.to_string();
    let temp = temp.as_bytes();
    file.write_all(temp).expect("ll");
}

pub fn test_file_lock() {
    println!("测试文件锁");
}

pub fn test_u64() {
    //算力比以前小了，得警告到钉钉,上次算力393665900982731，此次算力393665901719181
    // 算力比以前小了，得警告到钉钉,上次算力490847812144240，此次算力490847813020016
    //防止计算溢出（attempt to subtract with overflow），加checked_add;saturating_add、减checked_sub、乖checked_mul、除checked_div、checked_rem为%求余
    let oldcap: u64 = 393665900982731;
    let cap: u64 = 393665901719181;
    let tem = oldcap.checked_sub(cap);
    match tem {
        Some(sub) => {
            println!("{}-{}={}", oldcap, cap, sub);
            let t = sub.checked_div(10);
        }
        None => {
            println!("无法相减");
            let a = 31u64;
            let b = 10u64;
            let d = a.checked_div(b);
            match d {
                Some(k) => println!("{}/{}={}", a, b, k),
                None => println!("{}除以{},不开？？？", a, b),
            }
        }
    }
    if oldcap > cap {
        println!("大于");
    } else {
        println!("不大于");
    }
}
