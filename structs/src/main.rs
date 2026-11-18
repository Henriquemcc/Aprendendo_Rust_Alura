fn main() {
    conta_corrente();
}

struct Titular {
    nome: String,
    sobrenome: String,
}

struct Conta {
    titular: Titular,
    saldo: f64,
}

fn conta_corrente() {
    let conta: Conta = Conta {
        titular: Titular {
            nome: String::from("João"),
            sobrenome: String::from("Silva"),
        },
        saldo: 100.0,
    };

    println!(
        "Dados da conta: Titular = {} {}, Saldo = {}",
        conta.titular.nome, conta.titular.sobrenome, conta.saldo
    );
}
