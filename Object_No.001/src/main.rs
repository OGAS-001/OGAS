//use std::io;
use mysql::prelude::*;
use mysql::*;


#[derive(Debug)]
struct User {
    id: u32,
    username: String,
    info: String,
    group: u32,
}

impl User {
    fn create(id: u32, username: String, info: String, group: u32) -> User {
        User {
            id,
            username,
            info,
            group,
        }
    }
}

fn main(){
    let mut users: Vec<User> = Vec::new();
    //let u1 = User::create(0, "123".to_string(), "123".to_string(), 20213);
    //println!("{}", u1.id);
    /*
    2022.09.13
    测试成功,可以正常运行
    Debian: librust-curl-sys+openssl-sys-dev
    Arch: pkg-config openssl
    */

    let url: &str = "mysql://root:my-secret-pw@127.0.0.1:3306/test";
    let pool: Pool = Pool::new(url).unwrap(); // 获取连接池
    let mut conn = pool.get_conn().unwrap(); // 获取链接
    conn.query_iter("SELECT * from test_table")
        .unwrap()
        .for_each(|row| {
            let r: (u32, String, String, u32) = from_row(row.unwrap());
            let u: User = User::create(r.0, r.1, r.2, r.3);
            users.push(u);
        });
    for i in users {
        //println!("{},{},{},{}", i.id, i.username, i.info, i.group);
        println!("{:?}", i);
    }
    
}
