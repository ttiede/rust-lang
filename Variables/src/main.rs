fn main() {
    let an_integer = 1u;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {}", unit);

    // The compiler warns about unused variables; these warnings can be
    // silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u;
    let _noisy_unused_variable = 2u;
    println!("_unused_variable: {}", _unused_variable);
    println!("_noisy_unused_variable: {}", _noisy_unused_variable);
}