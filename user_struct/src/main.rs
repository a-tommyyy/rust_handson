struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

impl User {
    fn new(first_name: String, last_name: String, age: u8) -> User {
        User {
            first_name,
            last_name,
            age,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.last_name, self.first_name)
    }

    fn increment_age(&mut self) {
        self.age += 1
    }
}
fn main() {
    let mut user = User::new("タロウ".to_string(), "スタプラ".to_string(), 23);
    increment_userage(&mut user);
    let u1 = &user;
    let u2 = &user;
    print_username(u1);
    print_userage(u2);
}

fn increment_userage(user: &mut User) {
    user.increment_age()
}

fn print_username(user: &User) {
    println!("name: {}", user.full_name());
}

fn print_userage(user: &User) {
    println!("age: {}", user.age);
}
