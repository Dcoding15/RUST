/*
 * Variables: -
 * ---------
 * 1. We use 'let' keyword to declare variable.
 * 2. Bydefault rust variable is immutable.
 * 3. By using mut keyword we can change variable to mutable (Only change in value not in type).
 * 4. Shadowing means declare a new variable with same name as pervious variable (Redeclare same variable).
 * 5. Declaring constant also have to define value and type of constant variable.
 *
 * Naming Variables: -
 * ---It must start with either an alphabet or underscore.
 * 3. It only contain alphabet, digit or underscore.
 * 4. For constants use upper case alphabets only.
 * */

fn main()
{
    let _a = 10;                     // immutable variable - implicit declaration
    let mut b = 20;                 // mutable variable - implicit declaration
    let _c:i32;                     // _ as prefix use to ignore that variable - explicit  declaration
    const _PI:f64 = 3.14159265359;   // constant variable
    const _A:i32 = 3*3*3;

    assert_eq!(b, 20);              // It takes two argument and compare them
    //assert!()                     // It takes only condition as argument

    // Inner scope
    {
        // redeclaring values of same variable of outside scope
        b = 40;
        let _d: &str = "Inner scope";
        assert_eq!(b, 40);

        // taking values of same variable from outside of scope
        //assert_eq!(b, 20);
    }

    assert_eq!(b,40);                 // It get redefined from inner scope
    //assert_eq!(d, "Inner scope");   // Can't access inner scope variable directly
    
    // Type casting
    let e1 = 45_i32;                 // Value 45 is type cast to 32-bit int
    let e2:i64 = 50_i32 as i64;      // Value 50 of 32-bit int is type cast into i64
    
    // To define type of variablea
    type_of(&e1);
    type_of(&e2);
}

fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
