mod enums;
mod for_while_loop;
mod function;
mod if_else;
mod map;
mod matches;
mod mutability;
mod option;
mod result;
mod slice;
mod string;
mod struct_impl_trait;
mod variables;
mod vector;

fn main() {
    variables::run();
    function::run();
    mutability::run();
    slice::run();
    string::run();
    if_else::run();
    for_while_loop::run();
    matches::run();
    struct_impl_trait::run();
    enums::run();
    vector::run();
    map::run();
    option::run();
    result::run();
}
