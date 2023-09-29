use proji::{
    model::user::NewUser,
    provider::user::{create_user, delete_user},
};

fn main() {
    let new_user = NewUser {
        password: "987",
        username: "987",
    };
    let _ = new_user;

    let new_user = create_user(&new_user);
    match new_user {
        Ok(v) => println!("user : {:?} created !", v),
        Err(_) => println!("create user failed!"),
    }

    // delete user
    // let user = delete_user(3);
    // match user {
    //     Ok(v) => println!("user : {:?} deleted !", v),
    //     Err(_) => println!("delete user failed!"),
    // }

    // get user
    // let user = get_user(5);
    // match user {
    //     Ok(v) => println!("its list of items {:?} !", v),
    //     Err(_) => println!("update user failed!"),
    // }

    // get users
    // let users = get_users_list(20, 0);
    // match users {
    //     Ok(v) => {
    //         println!("its list of items {:?} !", v);
    //         for item in v.into_iter() {
    //             println!("updatet user is {} with id : {}", item.username, item.id)
    //         }
    //     }
    //     Err(_) => println!("update user failed!"),
    // }
}
