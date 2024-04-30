/*
 * Variables: -
 * ---------
 * 1. We use let keyword to declare variable.
 * 2. Bydefault rust variable is immutable.
 * 3. By using mut keyword we can change variable to mutable (Only change in value not in type).
 * 4. Shadowing means inner scope variable can access outer scope variable &  outer scope variable can't access inner scope variable.
 * 5. Declaring constant also have to define type of constant variable.
 * */

fn main()
{
    let a = 10;                 // immutable variable
    let mut b = 20;             // mutable variable
    const PI:f64 = 3.14159265359;   // constant variable
    const A:i32 = 3*3*3;
}
