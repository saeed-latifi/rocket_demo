use proji::provider::books::get_books_by_user_id;

fn main() {
    // create user
    // let new_user = UserNew {
    //     password: "987",
    //     username: "987",
    // };
    // let _ = new_user;

    // let new_user = create_user(&new_user);
    // match new_user {
    //     Ok(v) => println!("user : {:?} created !", v),
    //     Err(_) => println!("create user failed!"),
    // }

    // let update_info: UserUpdate = UserUpdate {
    //     password: Some("98754"),
    //     username: Some("wow"),
    // };

    // let updated_user = update_user(4, &update_info);
    // match updated_user {
    //     Ok(v) => println!("user : {:?} updated !", v),
    //     Err(e) => println!("update user failed! {}", e),
    // }

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
    //         // println!("its list of items {:?} !", v);
    //         for item in v.into_iter() {
    //             println!("updated user is {} with id : {}", item.username, item.id)
    //         }
    //     }
    //     Err(_) => println!("update user failed!"),
    // }

    let books = get_books_by_user_id(1);

    match books {
        Ok(res) => {
            for book in res.into_iter() {
                println!("{:?}", book)
            }
        }

        Err(e) => println!("err : {:?}", e),
    }
}
