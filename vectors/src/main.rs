fn main() {
    vectors();
}

fn vectors() {
    //let mut notas: Vec<f32> = Vec::new();
    let mut notas: Vec<f32> = vec![10.0, 8.8, 6.5];
    // notas.push(10.0);
    // notas.push(8.8);
    // notas.push(6.5);

    println!("{:?}", notas);

    notas.push(6.8);
    println!("{:?}", notas);

    println!("Nota 1 = {}", notas[0]);
    // println!("Nota 8 = {}", notas[7]); // Vai gerar um panic, pois não existe o indice 7 no vetor

    println!(
        "Nota 6 = {}",
        match notas.get(7) {
            Some(n) => *n,
            None => 0.0,
        }
    );

    println!(
        "Nota 4 = {}",
        match notas.get(3) {
            Some(n) => *n,
            None => 0.0,
        }
    );
}
