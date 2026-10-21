const PI: f32 = 3.14;
static VARIAVEL_GLOBAL: i8 = -1;

fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra() {
    let a = 123;
    
    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777; // Sombra a variável a declarada no escopo externo
        println!("dentro, a = {}", a);
    }

    // println!("fora, b = {}", b); // Não compila, pois b está fora do escopo no qual foi declarado
    println!("fora, a = {}", a);
}

fn escopo()
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

    println!("PI = {}", PI);

    println!("VARIAVEL_GLOBAL = {}", VARIAVEL_GLOBAL);
}

fn condicionais() {   
    let idade: u8 = 19;
    let responsavel_autorizou: bool = true;
    let eh_maior = idade >= 18;

    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade >= 16 && responsavel_autorizou {
        println!("Pode entrar na balada com assinatura do responsável");
    }
    else {
        println!("Não pode entrar na balada");
    }

    let condicao = if eh_maior { "maior" } else { "menor" };
    
    println!("É {} de idade", condicao);
}

fn main() {
    sombra();
    escopo();

    println!("Soma = {}", soma(2, 2));

    condicionais();
}