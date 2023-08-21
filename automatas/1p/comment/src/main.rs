use std::io;

#[allow(unused_variables)]
fn check_comment(s: &String) -> bool {
    let t1 = &s[..2];
    let t2 = &s[s.len() - 2..];
    if t1 == "/*" && t2 == "*/" {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut input = String::new();
    println!("Ingrese un comentario: ");
    io::stdin().read_line(&mut input).expect("Error");
    input = input.trim().to_string();
    if check_comment(&input) {
        println!("Comentario aceptado");
    } else {
        println!("No es un comentario");
    }
}
