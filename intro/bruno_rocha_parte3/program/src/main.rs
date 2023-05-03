fn main() {
    //primitives types
    //let x: u8 = 5; // OU let x = 5_u8;
    //let y: u8 = x - 20;
    //println!("{}", y)

    //##################################//
    //tuplas
    //let mut numbers = (1, 2, 3.5);
    //numbers.0 = 50;
    //numbers = (24, 42, 52.1);
    //let (a, b, c) = numbers;
    //println!("{:?}", numbers);

    //##################################//
    //arrays
    //let numbers: [i32;3] = [ 1, 2, 3 ];
    let mut numbers = [ 1.2, 2.3, 3.2 ];
    numbers[0] = 50.3;
    //slice
    let slice = &numbers[..2];//&numbers[1..2];

    println!("{:?}", slice);

}
