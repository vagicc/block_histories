use crate::db::MysqlPooledConnection;
use crate::models::coin_pool_model::get_spaces_count;
use crate::models::mass_block_histories_model::AddMassBlockHistories;

use std::process::Command;

pub fn devide_mass(new_block: &mut AddMassBlockHistories, connection: &MysqlPooledConnection) {
    // println!("/* spaces_count 总算力 SUM,  ratioData分配比例数据  block要分币的爆块  */");

    // let kda = new_block.pool_type.as_ref().unwrap().to_owned();
    let tx_id = &new_block.tx_id;
    let two_arg = format!("crontab/automass/{}", tx_id);
    let one_arg = "/www/wwwroot/bzz.mingyou168.cn/public/index.php";

    println!("执行的分币命令： php {} {}", one_arg, two_arg);

    /* 执行命令，使用PHP去分币 */
    let mut output = Command::new("php")
        .arg(one_arg)
        .arg(two_arg)
        .output()
        .expect("执行命令失败");

    println!("执行自动分币命令结果：{:#?}", output);
    //php nine.php crontab/automass dadsf
}
