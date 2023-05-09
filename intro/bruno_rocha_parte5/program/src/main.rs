use std::io;

fn main() {
    println!("{:-^40}", "Calculadora");
    //let letter: &str = "Daniel"; //str
    //string slice, string reference
    
    //heap String
    //String Dinâmica
    //String
    //let mut s = String::new();
    //s.push('D');
    //s.push('a');
    //s.push('n');
    //s.push('i');
    //s.push('e');
    //s.push('l');
    //s.push_str(" Aquino");
    //println!("{s}");

    //let s = "Daniel".to_string();

    //let mut s: String = "Daniel".into();
    //s = "teste".to_string();
//
    //println!("{s}")

    //use import
    let mut s = String::new();
    let banner = "\
    Digite uma sequêmcia de números \
    separados por vírgula \
    exemplo: 1,2,3,4,67\
    ";
    println!("{banner}");
    //Chama o io, o objeto do tipo stdin e a funcao read_line para fazer a leitura de linha no terminal
    io::stdin().read_line(&mut s).expect("Error reading console!"); // retorna um Result - contendo um valor ou um erro
    //println!("Você digitou {}", s.to_uppercase());
    //println!("Você digitou {}", s.replace("D", "x"));


    //calculadora
    let nuns: Vec<i32> = s.split(",").map(|c| c.trim().parse().expect("Error")).collect();
    let result: i32 = nuns.iter().sum();
    println!("O Total é:  {:?}", result);
    println!("{}", "-".repeat(40));
    //println!("Quantidade de letras {}", s.trim().chars().count());

}
