
fn main() { 
    let total = 30;
    {//inicio escopo
        let total = total + 20;
        println!("Trabalhou {} horas", total);
    }//fim escopo
    

    println!("Trabalhou {} horas", total);
    
}

