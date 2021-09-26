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
