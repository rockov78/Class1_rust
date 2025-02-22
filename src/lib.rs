pub fn calcular_promedio_movil(numeros: &Vec<i32>, ventana: usize) -> Vec<f64> {
    let mut promedios = Vec::new();
    if ventana == 0 || ventana > numeros.len() {
        return promedios;
    }

    for i in 0..=(numeros.len() - ventana) {
        let suma: i32 = numeros[i..i + ventana].iter().sum();
        let promedio = suma as f64 / ventana as f64;
        promedios.push(promedio);
    }

    promedios
}