
#[cfg(test)]
mod tests {
    use rust_algoritm_example::suma::suma;

    #[test]
    fn test_suma() {
        assert_eq!(suma(2, 2), 4);
        assert_eq!(suma(0, 0), 0);
        assert_eq!(suma(-1, 1), 0);
        assert_eq!(suma(100, 200), 300);
    }

    #[test]
    fn test_suma_numeros_negativos() {
        assert_eq!(suma(-5, -3), -8);
        assert_eq!(suma(-10, -20), -30);
    }

    #[test]
    fn test_suma_grande() {
        assert_eq!(suma(1000, 2000), 3000);
    }

    #[test]
    fn test_suma_cero() {
        assert_eq!(suma(0, 0), 0);
        assert_eq!(suma(42, -42), 0);
    }

    #[test]
    fn test_integracion_suma() {
        // Probamos diferentes combinaciones de números
        assert_eq!(suma(10, 20), 30);
        assert_eq!(suma(-10, 10), 0);
        assert_eq!(suma(0, 0), 0);
    }

    #[test]
    fn test_integracion_suma_grandes_numeros() {
        assert_eq!(suma(1000000, 2000000), 3000000);
    }
}