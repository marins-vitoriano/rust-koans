// Os elementos de um array podem ser acessados ​​pelos seus índices
//arr[4]
#[test]
fn array_index() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr[4] == 1);
}

// Um ​​novo array de tamanho fixo pode ser criado declarando o tipo de seus elementos
//junto com seu comprimento
// [i32; 0] = []
#[test]
fn array_empty() {
    let arr: [i32; 0];
    assert!(arr.len() == 0);
}


//Tentativa de acessar um array em um índice que é
// fora de seus limites causará um erro. Vamos causar
// esse erro neste exemplo.
#[test]
#[should_panic]
#[allow(const_err)]
fn out_of_index() {
    let arr: [&'static str; 5] = ["rust", "is", "mostly", "for", "nerds"];
    arr[__];
}

// Os elementos podem ser substituídos em um array em um determinado índice.
// dica: sem a palavra-chave 'mut', você não poderá alterar os dados.
#[test]
fn insert_at_index() {
    let mut arr: [u8; 5] = [0, 1, 2, 3, 4];
    __ = 0;
    assert!(arr == [0, 1, 2, 3, 0]);
}

// Arrays podem ser iterados.
#[test]
fn array_iteration() {
    let arr: [u8; 3] = [3, 2, 1];
    let mut iterator = arr.iter();
    assert!(iterator.next().unwrap() == &__);
    assert!(iterator.next().unwrap() == &__);
    assert!(iterator.next().unwrap() == &__);
}

// Arrays também podem sofrer mutação durante a iteração
#[test]
fn array_map() {
    let arr: [u32; 4] = [2, 5, 7, 4];
    let mut iterator = arr.iter().map(__);
    assert!(iterator.next() == Some(4));
    assert!(iterator.next() == Some(10));
    assert!(iterator.next() == Some(14));
    assert!(iterator.next() == Some(8));
}

// Você pode filtrar um array por resultados que correspondam a uma determinada condição
#[test]
fn array_filter() {
    let arr: [u16; 5] = [1, 2, 3, 4, 5];
    let mut iterator = arr.iter().filter(__);
    assert!(iterator.next().unwrap() == &2);
    assert!(iterator.next().unwrap() == &4);
    assert!(iterator.next().is_none());
}

// Filtro e mapa podem ser combinados para fazer as duas coisas ao mesmo tempo
#[test]
fn array_filter_map() {
    let arr: [u8; 5] = [2, 1, 2, 1, 2];
    let mut iterator = arr.iter().filter_map(|&x| if x == 1 { Some(__) } else { None });
    assert!(iterator.next() == Some(3));
    assert!(iterator.next() == Some(3));
    assert!(iterator.next().is_none());
}

//Isso também pode ser usado para lógicas mais complexas
#[test]
fn complex_array_filter_map() {
    let arr: [u64; 4] = [4, 8, 16, 32];
    let mut iterator = arr.iter()
        .filter_map(|&x| if (x as f64).sqrt().floor() == (x as f64).sqrt() {
            Some((x as f64).sqrt() as u64)
        } else {
            None
        });
    assert!(iterator.next().unwrap() == __);
    assert!(iterator.next().unwrap() == __);
    assert!(iterator.next().is_none());
}

// Arrays também podem ser iterados usando um loop for
#[test]
fn for_loops() {
    let arr: [u64; 3] = [1, 2, 3];
    let mut y: u64 = 1;
    for x in &arr {
        assert!(*x == y);
        __
    }
}

// Vamos tentar iterar um array de strings para construir uma frase
#[test]
fn for_loops_two() {
    let words: [&'static str; 3] = ["I", "love", "Rust"];
    let mut sentence: String = String::new();
    for word in words.iter() {
        __
    }
    println!("{:?}", sentence);
    assert!(sentence == "I love Rust".to_string());
}
