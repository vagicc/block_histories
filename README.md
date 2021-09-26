 
这是一个处理币种收益提示消息到钉钉群的程序
日前支持mass、xfx、chia、cru四个币种
运行命令为：cargo run 币种
如mass:  cargo run mass

mass币种的处理是通过ssh远程登录到钱包机，执行命令获取到爆块收效信息。
xfx币种是通过处理矿机生成的json文件来获取到爆块收益信息
chia是通过爬虫抓取奇亚官方区块浏览器html，分析得到处理js文件，再分析js文件里的请求接口域名，再通过域名接口获取爆块收益

---------------------------------------------------------------------------------------------
 
 精通比特币：https://wizardforcel.gitbooks.io/masterbitcoin2cn/content/

-------------------------------数据库迁移使用说明------------------------------------------------
这是一个用Rust中的warp框架中使用diesel的示例

# diesel是Rust的ORM(对象关系映射器)和查询构建器
# diesel为PostgreSQL、Mysql及SQLite提供了开箱即用的支持

diesel数据库迁移使用说明
diesel是Rust的ORM(对象关系映射器)和查询构建器
diesel为PostgreSQL、Mysql及SQLite提供了开箱即用的支持
diesel-cli命令行工具（创建、迁移）：

安装diesel-cli工具(postgres)：cargo install diesel_cli --no-default-features --features postgres
PostgreSQL错误解决的：sudo apt-get install libpq-dev

安装diesel-cli工具(mysql)：cargo install diesel_cli --no-default-features --features mysql
mysql错误解决：sudo apt-get install libmysqlclient-dev

在cargo项目根目录下添加.env文件,加下如下条进行连接配置：
postgres数据库：
DATABASE_URL=postgres://postgres:llxxs@127.0.0.1:5432/linksnap
mysql数据库：
DATABASE_URL=mysql://[user[:password]@]host/database_name

在Cargo.toml中添加依赖项：
diesel = { version="1.4.6",features=["extras","postgres","r2d2"] }
dotenv = "0.15.0"

运行"diesel setup"命令生成"migrations"目录与"diesel.toml"文件并且会创建数据库：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel setup
Creating migrations directory at: /luck/Language/Rust/warp-wiki/migrations
Creating database: warpwiki
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$

创建admins表迁移，运行创建表迁移命令（diesel migration generate 表名）：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel migration generate admins
Creating migrations/2021-05-13-071702_admins/up.sql
Creating migrations/2021-05-13-071702_admins/down.sql
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ 
命令运行后会生成两个空的迁移文件up.sql和down.sql,
迁移文件只是普通的SQL,接着在up.sql上面添加CREATE TABLE,同时在down.sql添加相应的DROP TABLE

执行表迁移命令（diesel migration run）：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel migration run
Running migration 2021-05-13-071702_admins
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$
命令执行完后，会在数据库中生成表，同时在项目中生成src/schema.rs文件。


迁移时执行：diesel setup

运行时出错：error while loading shared libraries: libmariadb.so.3: cannot open shared object file: No such file or directory
解决： sudo ln -s /usr/local/mysql/lib/libmariadb.so.3 /usr/lib/libmariadb.so.3

-------------------------“重做”应用的迁移------------------------------------------
应用迁移：diesel migration run  
恢复迁移：diesel migration revert
“重做”应用的迁移：
          diesel migration revert
          diesel migration run
重做（等同于上面两条）：diesel migration redo
上面命令，只能运行、还原或重做一次迁移
重做所有迁移：diesel migration redo all
-------------------------------------------------------------------------------

===============================================================================
查询条件看：src/expression_methods/global_expression_methods.rs
事务查看：
    src/connection/mod.rs  事务写在一个函数的示例
    src/doctest_setup.rs
    src/connection/transaction_manager.rs  事务
    src/pg/transaction.rs   pg数据库特有的事务
===============================================================================



                    单点登录原理 jwt token 令牌
​刚才突然想明白了单点登录的token为什么不需要针对每个用户都存一个session。

服务器这边准备一个密码字符串，各服务器保持字符串一样。

当用户登录一台服务器成功后，服务器将他的账号和密码字符串通过算法整合一下，形成一个新的字符串。通常这个算法会将密码字符串用哈希的方式来处理(即在新的字符串中不知道这个密码是多少)，但账号可以直接在新的字符串中拿到。

这样，服务器将这个新的字符串给用户(即保存在cookie中，或保存在storage中)。用户每次访问，都会带上这个字符串(类似jsessionid)。

服务器这边拿到用户带来的字符串，从中取出账号，将账号和服务器这边存的密码用相同算法整合，形成新的字符串，用这个字符串和用户带来的字符串比对，看是否相同，相同，即通过，不同，让用户去登录。

953.7GB   

193.7GB nft
180GB   windows 10
580GB   XUbuntu



