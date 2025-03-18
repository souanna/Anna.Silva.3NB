// src/lib.rs

/// Esta função soma os elementos de um array apontado por `ptr`
/// considerando que o array possui `len` elementos.
///
/// # Segurança
///
/// - O ponteiro `ptr` deve ser válido e apontar para um array com pelo menos `len` elementos.
/// - **Atenção:** Esta implementação possui um erro intencional!
pub unsafe fn sum_array(ptr: *const i32, len: usize) -> i32 {
    let mut sum = 0;
    // BUG: o loop percorre de 0 até len inclusive (0..=len)
    for i in 0..=len {
        // Utiliza o offset para acessar cada elemento
        sum += *ptr.offset(i as isize);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_array() {
        // Cria um array de inteiros
        let arr = [1, 2, 3, 4, 5];
        // Chama a função sum_array de forma insegura
        let sum = unsafe { sum_array(arr.as_ptr(), arr.len()) };
        // A soma correta dos elementos é 15
        assert_eq!(sum, 15);
    }
}

fn main() {
    // Para executar com "cargo run" e observar o comportamento,
    // descomente as linhas abaixo:
}