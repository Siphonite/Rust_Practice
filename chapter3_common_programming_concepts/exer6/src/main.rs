//Assign different literals (integer, float, char, boolean) and print their types using std::any::type_name::<T>().

use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let int_literal = 42;
    let float_literal = 3.14;
    let char_literal = 'R';
    let bool_literal = true;

    println!("int_literal: {}", type_of(&int_literal));
    println!("float_literal: {}", type_of(&float_literal));
    println!("char_literal: {}", type_of(&char_literal));
    println!("bool_literal: {}", type_of(&bool_literal));
}