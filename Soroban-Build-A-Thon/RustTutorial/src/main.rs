fn main() {

//====================================================
// ========= Rust Tutuorial part 1

    // Variables
    let a: i32 = 16;
    println!("{}", a);

    // Strings
    let z: String = String::from("Hello, Soroban");
    let y: &str = "Hello Stellar!";
    println!("{y}");
    println!("{z}");
    
    // functions
    pub fn event (_name: String) {
        let name: String = String::from("James");
        println!("{}", name);
    }

    let _e: EventForKids = EventForKids {
        name: String::from("KidsCo"),
        date: String::from("04.03.2024"),
        number_of_participants: 1000,
        place: String::from("NY, USA")
    };

}

//=================================================
//=========== Part 1 Structs and enums

// Structs
// compiling data types in one class
struct EventForKids {
    name: String,
    date: String,
    number_of_participants: u32,
    place: String
}

// Enums
// compiling errors in one class
enum ErrorsForEvents {
    NoEvent,
    CancelledEvent,
    EventType
}
