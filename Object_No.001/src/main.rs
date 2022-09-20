use mysql::*;
use mysql::prelude::*;


fn main(){
    /*
    2022.09.13
    测试成功,可以正常运行
    Debian系列可能会需要安装:librust-curl-sys+openssl-sys-dev
    */
    let url = "mysql://root:password@ipaddr:port/database";
    let pool = Pool::new(url).unwrap(); // 获取连接池
    let mut conn = pool.get_conn().unwrap();// 获取链接
    conn.query_iter("SELECT * from userinfo").unwrap().for_each(|row|
    {
        let r: (u8,String,String,String) = from_row(row.unwrap());
        println!("{}, {},{},{}", r.0, r.1, r.2, r.3);
    });



}
