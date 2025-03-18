fn main() {
    let x = 10; // `x` é o dono do valor 10
    let y = &x; // `y` é uma referência imutável para `x`
    println!("Valor de x: {}, via referência: {}", x, y);

    // Para modificar o valor, precisamos de uma referência mutável:
    let mut z = 20;
    {
        let w = &mut z; // Empréstimo mutável; só pode existir uma referência mutável ativa
        *w += 5; // Desreferenciamos `w` para alterar o valor
    }
    println!("Novo valor de z: {}", z);
}