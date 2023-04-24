struct UserAccount {
    name: String,
    age: Option<u32>
}

trait Balance {
    fn get_balance(&self) -> u32 {
        return 10;
    }
}

impl Balance for UserAccount {
}

fn increase_balance<T: Balance>(bal: &T, amt: u32) -> Result<u32, String>{
    if amt <= 10 {
        return Ok(bal.get_balance() + amt);
    }
    return Err(String::from("Increase must be less than 10!"));
}

fn main() {
    let user = UserAccount {
        name: String::from("Benjamin"),
        age: Some(37),
    };

    if let Some(age) = user.age {
        println!("User age: {age}");
    }

    match increase_balance(&user, 11) {
        Ok(amt) => println!("UserAccount balance increased to {}", amt),
        Err(e) => println!("{}", e),
    }
    
    if let Some(age) = user.age {
        println!("User age: {age}");
    }
 }