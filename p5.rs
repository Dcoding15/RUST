/*
 * 1. Without return and parameter  : fn function_name() {}
 * 2. With return and parameter     : fn function_name(variable1: type1, variable2: type2, ..., variableN: typeN) -> retun_type {  // code; return_type; }
 *
 * */

// User function declared and defined here

fn msg()
{
    println!("This is hello message");
}

fn welcom(name: &str)
{
    println!("Hello, {name}");
}

fn rect_area(a: i32, b: i32) -> i32
{
    return a*b;
}

// Main function

fn main()
{
    // Function can called inside main function
    msg();
    welcom("Debrishi");
    let a = rect_area(10,20);
    println!("{a}");
}
