pub fn run() {
    let name = "Thomas";

    /**
     *In Rust, all variables which are assigned using a let statement are immutable unless otherwise specified.
     *Immutable is a fancy term which means:

    Fix - add 'mut' to variable
     */
    // name = "Jane";
    println!("Name is: {}", name);

    // references
    let mut name1 = "Thomas";
    name1 = "Rose";


    // Even though [name1] is mutable, reference to it will always be immutable, unless specified.
    // The reference type is completely independent of the original variable’s data type…
    // This means we can create an immutable reference to a mutable variable

    let name1_ref = &name1;


    //Although we cant have mutable references to an immutable type. Like, below will throw error
    // let name2 = "Thomas";
    // let name2_ref = &mut name2;


    // ----------------------------------------------------------------

    let mut our_player = Player
    {
        name: "Jones".to_string(), //converts &str (static string) to a String (heap memory)
        age: 25,
        description: "Just a happy guy.".to_string(),
    };


    // Throws error - use of moved value: our_player.name
    // Since we specify that mover() takes a Player and not a &Player (immutable reference to Player),
    // Rust transfers ownership of our_player to mover() permanently.
    // This means that main() no longer has any control over or access to our_player
    // mover(our_player);
    // println!("My name is {}, and I am being used after a move", our_player.name);


    mover(&our_player);

    let age: u8 = 55;
    mover2(age);

    // This works fine. Because some types have implemented Copy trait. For members that have this trait, compiler
    // does not move its ownership to another entity, instead it is copied.

    // Primitives have this trait by default
    println!("The age is: {}", age);


    // Borrowing
    // Immutable borrows is same as above, where we send immutable reference to another function

    let mut our_player_new = Player
    {
        name: "Jones".to_string(),
        age: 25,
        description: "Just a happy guy.".to_string(),
    };

    immutable_borrow(&our_player_new);
    change_name(&mut our_player_new);
    println!("My name is {}, and I am being used after an mutable borrow", our_player_new.name);


    //Here, we’ve immutably borrowed our_player into my_immutable_return.
    // We’ve then tried to mutably borrow our_player while it is stilled immutably borrowed.
    // It does not get “returned to its owner” until the curly brace after it. The problem here is that
    // my_immutable_return has borrowed my_player until the curly brace at the end of the main() function.

    // let my_immutable_return = immutable_borrow_with_return(&our_player);
    // change_name(&mut our_player);
    // println!("My name is {}, and I am being used after an mutable borrow", our_player.name);
    // Fix - put my_immutable_return inside another {} block. and rest remains same



    // C & C++  - & means pass by reference, and no & means pass by value
    // Rust - & - pass by borrow, and no & is pass by move
}


// - - - - -- - - - - - - -- - --  - - - - -- - --  - - - - -- - --  - - - - -- - --  - - - - -- - --

fn immutable_borrow_with_return(borrowed: &Player) -> &Player
{
    println!("I am {}, I've been immutably borrowed", borrowed.name);
    borrowed
}

fn immutable_borrow(borrowed: &Player) {
    println!("I am {}, I've been immutably borrowed", borrowed.name);
}

fn change_name(borrowed: &mut Player) {
    borrowed.name = "My New Name".to_string();
}

fn mover2(age: u8) -> u8 {
    println!("Age {} has been moved into mover!", age);
    age
}


fn mover(moved: &Player) {
    println!("I am {}, I've been moved into mover", moved.name);
}

struct Player {
    name: String,
    age: u8,
    description: String,
}
