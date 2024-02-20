fn main() {
    // let user_1 = Person {
    //     first_name: String::from("Daniel"),
    //     last_name: String::from("Johnson"),
    //     age: 25,
    //     email: String::from("tenkorangd5@gmail.com"),
    //     is_active: true,
    // };

    // let mut user2 = Person {
    //     first_name: String::from("Kwame"),
    //     last_name: String::from("Kyei"),
    //     age: 23,
    //     email: String::from("kwamekyeimonies@gmail.com"),
    //     is_active: true,
    // };
    let user_email = String::from("tenkorangd5@gmail.com");
    let first_name = String::from("Daniel");
    let last_name = String::from("Tenkorang");
    let age = 25;

    let user = build_person(user_email, first_name, last_name, age);

    println!("{:?}", user);
}


#[derive(Debug)] 
struct Person {
    first_name: String,
    last_name: String,
    age: u64,
    email: String,
    is_active: bool,
}

fn build_person(email: String, first_name: String, last_name: String, age: u64) -> Person {
    Person {
        first_name: first_name,
        last_name: last_name,
        age: age,
        email: email,
        is_active: true,
    }
}
