/// 取得.env文件key里的值
pub fn get_env(key: &str) -> String {
    use dotenv::dotenv;
    use std::env;
    dotenv().ok();
    let msg = "env文件必须配置的环境变量：".to_string() + key;
    let value = env::var(key).expect(&msg);
    value
}

pub fn get_ssh_session(ip: &str) -> ssh2::Session {
    use ssh2::Session;
    use std::net::TcpStream;
    use std::path::Path;

    let pubkey = Path::new("/root/.ssh/id_rsa.pub");
    let privatekey = Path::new("/root/.ssh/id_rsa");

    let alert = format!("远程连接矿机{}出错", ip);
    let tcp = TcpStream::connect(ip).expect(&alert);
    let mut session = Session::new().unwrap();

    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session
        .userauth_pubkey_file("root", Some(pubkey), privatekey, None)
        .expect(&alert);

    if !session.authenticated() {
        println!("登录矿机失败");
    }

    session
}

/*  */
pub fn get_ssh_session_passwd(ip: &str, user: &str, passwd: &str) -> ssh2::Session {
    use ssh2::Session;
    use std::net::TcpStream;
    use std::path::Path;

    let tcp = TcpStream::connect(ip).expect("出错");
    let mut session = Session::new().unwrap();

    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password(user, passwd).expect("msg");

    if !session.authenticated() {
        println!("登录矿机失败");
    }

    session
}

/* 管理机登录矿机 */
pub fn login_server(
    session: &mut ssh2::Session,
    ip: &str,
    user: &str,
    passwd: &str,
) -> ssh2::Session {
    use ssh2::Session;
    use ssh2::Stream;
    use std::net::TcpStream;
    use std::path::Path;


    println!("这里1kk==");
    let tcp = TcpStream::connect(ip).expect("msg");
    println!("这里22222222kk==");

    session.set_tcp_stream(tcp);
    println!("这里3333333333==");

    session.handshake().unwrap();
    session.userauth_password(user, passwd).expect("msg");
    if !session.authenticated() {
        println!("登录矿机失败");
    }
    session.clone()
}
