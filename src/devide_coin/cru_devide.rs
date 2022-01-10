use crate::db::MysqlPooledConnection;
use crate::models::cru_block_histories_model::AddCruBlock;
use std::process::Command;

/* 这里只有当爆块程序放在和PHP同一台服务器上时才可以使用 */
pub fn php_devide_cru(new_block: &AddCruBlock, connection: &MysqlPooledConnection) {
    let extrinsic_index = &new_block.extrinsic_index;
    let mut event_idx = 0;
    match &new_block.event_idx {
        Some(idx) => event_idx = *idx,
        None => {}
    }

    let two_arg = format!("crontab/autoCru/{}/{}", extrinsic_index, event_idx);
    let one_arg = "/www/wwwroot/bzz.mingyou168.cn/public/index.php";

    println!("执行的分币命令： php {} {}", one_arg, two_arg);

    /* 执行命令，使用PHP去分币 */
    let mut output = Command::new("php")
        .arg(one_arg)
        .arg(two_arg)
        .output()
        .expect("执行命令失败");

    println!("cru执行自动分币命令结果：{:#?}", output);
}
