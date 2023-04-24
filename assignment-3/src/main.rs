// define struct of UserAccount with field: name (String), and age (Option<u32>)
struct UserAccount {
    name: String,
    age: Option<u32>
}

// define a trait called Balance, and within, function get_balance returning integer of 10

trait Balance {
    fn get_balance(&self) -> u32 {
        return 10;
    }
}

// implement trait Balance to UserAccount struct
impl Balance for UserAccount {
}

// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
// within this function,
// - if increase amount is <= 10, return an OK containing the get_balance + amount
// - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
// Tip: this function should return a Result<u32, String>

fn increase_balance<T: Balance>(bal: &T, amt: u32) -> Result<u32, String>{
    if amt <= 10 {
        return Ok(bal.get_balance() + amt);
    }
    return Err(String::from("Increase must be less than 10!"));
}

fn main() {
    // create user_account, and set his age as Option::None
    let user = UserAccount {
        name: String::from("Benjamin"),
        age: Some(37),
    };

    // use an if...let...else statement to print the UserAccount age if it is a Option::Some
    if let Some(age) = user.age {
        println!("User age: {age}");
    }

    // You want to increase the user_account's balance by 11
    // use a match, if the result of increase_balance is
    // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
    // - Err: print the error message returned
    match increase_balance(&user, 11) {
        Ok(amt) => println!("UserAccount balance increased to {}", amt),
        Err(e) => println!("{}", e),
    }
    
    // use an if...let...else statement to print the UserAccount age if it is a Option::Some
    if let Some(age) = user.age {
        println!("User age: {age}");
    }

 }