const PI: f32 = 3.14;
static VARIAVEL_GLOBAL: i8 = -1;

fn main()
{
    let variavel: u8 = 128;
    println!("variavel = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

    let decimal: f32 = 2.5;
    println!("decimal = {}, tamanho = {}", decimal, std::mem::size_of_val(&decimal));

    let mut booleana: bool = false;
    println!("booleana = {}, tamanho = {}", booleana, std::mem::size_of_val(&booleana));

    booleana = true;
    println!("booleana = {}, tamanho = {}", booleana, std::mem::size_of_val(&booleana));

    let letra: char = 'z';
    println!("letra = {}, tamanho = {}", letra, std::mem::size_of_val(&letra));

    println!("PI = {}, tamanho = {}", PI, std::mem::size_of_val(&PI));

    println!("VARIAVEL_GLOBAL = {}, tamanho = {}", VARIAVEL_GLOBAL, std::mem::size_of_val(&VARIAVEL_GLOBAL))
}