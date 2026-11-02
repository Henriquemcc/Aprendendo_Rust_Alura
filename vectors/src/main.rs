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
}
