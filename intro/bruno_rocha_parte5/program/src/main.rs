use std::io;

fn main() {
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
    println!("Digite um texto");
    //Chama o io, o objeto do tipo stdin e a funcao read_line para fazer a leitura de linha no terminal
    io::stdin().read_line(&mut s).expect("Error reading console!"); // retorna um Result - contendo um valor ou um erro
    println!("Você digitou {s}");

}
