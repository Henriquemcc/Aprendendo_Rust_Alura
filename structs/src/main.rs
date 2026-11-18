fn main() {
    conta_corrente();
}

struct Conta {
    titular: String,
    saldo: f64,
}

fn conta_corrente() {
    let conta: Conta = Conta {
        titular: String::from("João"),
        saldo: 100.0,
    };

    println!(
        "Dados da conta: Titular = {}, Saldo = {}",
        conta.titular, conta.saldo
    );
}
