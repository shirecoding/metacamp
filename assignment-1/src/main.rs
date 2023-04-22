struct User {
    name: String,
    balance: (f32, String)
}

impl User {
    fn print_user_detail(&self) {
        println!("name: {} balance: {} currency: {}", self.name, self.balance.0, self.balance.1);
    }
}

fn accrue_interest(user: &mut User, interest: f32) {
    user.balance.0 += interest * user.balance.0 / 100.0;
    user.print_user_detail();
}

fn main() {
    let mut user = User {
        name: "Benjamin".to_owned(),
        balance: (100.0, "SGD".to_owned())
    };
    user.print_user_detail();
    accrue_interest(&mut user, 10.0);
    for _ in 1..10 {
        accrue_interest(&mut user, 10.0);
    }
}
