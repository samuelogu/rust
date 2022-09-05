//Traditional struct
/*struct Color {
    red: u8,
    green: u8,
    blue: u8
}*/

//Tuple struct
// struct Color(u8, u8, u8);

    struct Person {
        first_name: String,
        last_name: String
    }

    impl Person {
        fn new(first: &str, last: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string()
            }
        }

        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }

        fn set_last_name(&mut self, last: &str) {
            self.last_name = last.to_string();
        }

        fn name_to_tuple(self) -> (String, String) {
            (self.first_name, self.last_name)
        }

    }

pub fn run() {
    /*let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);*/

    /*let mut c = Color(255, 0, 0);
    c.0 = 200;
    println!("Color: {} {} {}", c.0, c.1, c.2);*/

    let mut p = Person::new("Samuel", "Ogu");
    println!("Person: {} {}", p.first_name, p.last_name);
    p.set_last_name("James");

    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.name_to_tuple());





}
