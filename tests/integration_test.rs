use rust_algoritm_example;

#[test]
fn test_integracion_suma() {
    // Probamos diferentes combinaciones de números
    assert_eq!(rust_algoritm_example::suma(10, 20), 30);
    assert_eq!(rust_algoritm_example::suma(-10, 10), 0);
    assert_eq!(rust_algoritm_example::suma(0, 0), 0);
}

#[test]
fn test_integracion_suma_grandes_numeros() {
    assert_eq!(rust_algoritm_example::suma(1000000, 2000000), 3000000);
}