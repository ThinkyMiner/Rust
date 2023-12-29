struct  User{
    active:bool,
    username:String,
    roll_number:u32,
}

fn main() {
    let user = User{active : true, username: "Kartik".to_string(), roll_number:24};
    println!("{}, {}, {}", user.active, user.username, user.roll_number);
    let user2 = make_users("Sehgal".to_string(), true);
    println!("{}", user2.username);
}

fn make_users(usrname:String, boolean:bool) -> User{
    User{active:boolean, username:usrname,roll_number:4 }
}
