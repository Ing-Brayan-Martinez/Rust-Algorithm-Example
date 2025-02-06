

#[cfg(test)]
mod tests {
    use rust_algoritm_example::suma;

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
}