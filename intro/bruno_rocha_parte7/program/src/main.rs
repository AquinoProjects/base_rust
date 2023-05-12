fn main() {
    /*fn say_hello( name: &str, color: &str) {
        println!("{name} {color}")
    }

    let nameSay: &str = "Daniel";
    let myColor: &str = "red";
    
    let y: () = {
        say_hello(nameSay, myColor);
        let x = 5;
        99;

     };
     println!("{:?}", y);

     fn add_numbers(x:i32, y: i32) -> i32 {
        if x == 0 {
            return y;
        }
        return x + y;
     }
     let result: i32 = add_numbers(8,9);
     println!("{result}");*/

     fn double(n:i32) -> i32{
        n*2
     }

     fn convert_to_number(s: &str) -> i32 {
        s.parse().unwrap()
     }

     let input = "56 65 48 59 56 87 23";
     let result: Vec<i32>= input.split(" ").map(|s| s.parse::<i32>().unwrap()).map(|n| n * 2).collect();
     println!("{:?}", result)
}
