pub fn run() {

    // structs - custom data models
    let mut color = Color {
        red: 2,
        green: 4,
        blue: 4,
    };

    // because color is mut, we can mutate its members as well
    color.red = 2;

    println!("Colors {} {} {}", color.red, color.red, color.blue);

    let mut color_tuple = ColorTuple(255, 255, 255);

    color_tuple.0 = 0;

    println!("{}{}{}", color_tuple.0, color_tuple.1, color_tuple.2);

    let mut person = Person::new("Piyush", "Pratheep");

    person.set_last_name("Pradeepkumar");

    println!("Person is {}", person.get_full_name());

    println!("Person Tuple - {:?}", person.to_tuple());
}

struct Person {
    first_name: String,
    last_name: String,
}


impl Person {
    //Construct a person

    fn new(first_name: &str, last_name: &str) -> Self {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }


    fn set_last_name(&mut self, new_last_name: &str) {
        self.last_name = new_last_name.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

//Tuple struct
struct ColorTuple(u8, u8, u8);

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
}
