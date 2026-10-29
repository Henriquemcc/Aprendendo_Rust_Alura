fn rouba(string: &mut String) {
    // string = String::from("Oi"); // Não compila, pois string é uma referência imutável
    string.push_str(" Como vai?");
    println!("{}", string);
}

fn ownership() {
    let mut uma_string = String::from("Olá, mundo!");
    rouba(&mut uma_string);

    println!("{}", uma_string);
}

fn main() {
    ownership();
}
