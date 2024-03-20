fn main() {
    // tuple of lenght 3
    let tuple_a = ('A', 7u8, false);

    // classic struct with named fields
    struct Student {
        name: String,
        age: u8,
        remote: bool,
    }

    // tuple struct with data types only
    struct Grades(char, char, char, f32);

    // unit struct
    struct Unit;

    // using tuple indexing show all the alements of the tuple
    println!(
        "First {}, second {} third {}",
        tuple_a.0, tuple_a.1, tuple_a.2
    );

    // Instantiate the classic struct in random or specified order
    let user_a = Student {
        name: String::from("Kevin"),
        age: 17,
        remote: true,
    };
    let user_b = Student {
        remote: false,
        age: 19,
        name: String::from("Alice"),
    };

    println!(
        "{} is {} years old and is remote: {}",
        user_a.name, user_a.age, user_a.remote
    );
    println!(
        "{} is {} years old and is remote: {}",
        user_b.name, user_b.age, user_b.remote
    );

    // Instantiate the tuple struct
    let mark_a = Grades('A', 'B', 'C', 3.5);
    let mark_b = Grades('B', 'C', 'D', 2.5);

    println!(
        "Mark A: {}, {} and {} with a GPA of {}",
        mark_a.0, mark_a.1, mark_a.2, mark_a.3
    );
    println!(
        "Mark B: {}, {} and {} with a GPA of {}",
        mark_b.0, mark_b.1, mark_b.2, mark_b.3
    );
}
