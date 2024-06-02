// Booleanos podem ter dois valores, verdadeiro ou falso.
// Dois valores iguais retornarão verdadeiros quando comparados com o operador ==
#[test]
fn truth() {
    assert!(true == true);
}

// Da mesma forma, dois valores desiguais retornarão falso quando comparados com ==
// O operador != pode ser usado para retornar verdadeiro para uma inequação
#[test]
fn falsehood() {
    assert!(false != true);
}

// Strings também podem ser comparadas e retornarão um booleano
#[test]
fn string_equality() {
    assert!("Stuff" == "Stuff");
}

//Inteiros podem ser comparados desde que sejam do mesmo tipo
#[test]
fn int_equality() {
    let num: i8 = 5;
    assert!(num == 5);
}
