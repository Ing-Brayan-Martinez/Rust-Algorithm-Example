// Importamos las funciones del crate principal
use rust_algoritm_example::suma;

#[test]
fn test_suma_grande() {
    assert_eq!(suma(1000, 2000), 3000);
}

#[test]
fn test_suma_cero() {
    assert_eq!(suma(0, 0), 0);
    assert_eq!(suma(42, -42), 0);
}