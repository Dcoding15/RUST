/*
 * Data Types: -
 *  1. Scalar(Single Value): -
 *      Length      Signed      Unsigned
 *      ======      ======      ========
 *      1-bit                     bool
 *      1-bit                     char
 *      8-bit         i8          u8
 *      16-bit        i16         u16
 *      32-bit        i32         u32
 *      32-bit        f32
 *      64-bit        i64         u64
 *      64-bit        f64
 *      128-bit       i128        u128
 *      arch dept     isize       usize
 *      
 *      Integer Literals Perfix: -
 *      =======================
 *      Decimal     -> 
 *      Hexadecimal -> 0x
 *      Octal       -> 0o
 *      Binary      -> 0b
 *      Byte        -> b''
 *
 *      Numeric Operations: +   -   *   /   %
 *
 *      Note: -
 *          1. Number literal also can use visual seperator as _
 *
 *  2. Compound (Multiple Values): -
 *      a) Tuple: -
 *          i.      Tuple has fixed length i.e., they can't grow or shrink in size
 *          ii.     Tuple have different types of values
 *          iii.    Annotation for tuple is () with different values type
 *          iv.     These are stored inside () and separaed by ,
 *          v.      Tuple without values are known as unit.
 *          vi.     Access tuple through destructing and indexing
 *      b) Array: -
 *          i.      Array has fixed length i.e., they can't grow or shrink is size
 *          ii      Array has same types of values
 *          iii.    Annotation for array is [] with same types of values
 *          iv.     These are stored inside [] and separated by ,
 *          v.      Concise way to define and declared an array is [value / type; occurance]
 *          vi.     Access array through destructing and indexing
 * */

fn main()
{
    let a:isize = 12_36_547;    // Integer
    let b:f64 = 63.254;         // Floating point
    let c:bool = true;          // Boolean
    let d:char = '😎';          // Character

    println!("a={a}, b={b}, c={c}, d={d}");

    let bin = 0b10100;      // base 2 - binary
    let oct = 0o24;         // base 8 - octal
    let dec = 20;           // base 10 - decimal
    let hex = 0x14;         // base 16 - hexadecimal
    let byt = b'H';         // byte - u8

    println!("bin={bin}, oct={oct}, dec={dec}, hex={hex}, byt={byt}");

    let tpl: (i64,f64,bool,char) = (23,3.25,false,'D');     // Tuple
    let (w1,x1,y1,z1) = tpl;                                // Destructing

    println!("w1={w1}, x1={x1}, y1={y1}, z1={z1}");

    // Indexing
    let w2 = tpl.0;
    let x2 = tpl.1;
    let y2 = tpl.2;
    let z2 = tpl.3;

    println!("w2={w2}, x2={x2}, y2={y2}, z2={z2}");

    let arr = [1,2,3,4];        // Array
    let arr1 = [2; 10];         // [value, occurrence]
    let _arr2: [i32; 5];        // [type, occurence]
    let [w2,x2,y2,z2] = arr;    // Destructing

    println!("arr[0]={w2}, arr[1]={x2}, arr[2]={y2}, arr[3]={z2}");
}
