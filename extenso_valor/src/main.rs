use myrustlib::{self, get_f64};

const unidades: [&str; 10] = [
    "", "um", "dois", "três", "quatro", "cinco", "seis", "sete", "oito", "nove",
];
const dezenas: [&str; 10] = [
    "",
    "dez",
    "vinte",
    "trinta",
    "quarenta",
    "cinquenta",
    "sessenta",
    "setenta",
    "oitenta",
    "noventa",
];
const dezenas_unit: [&str; 10] = [
    "dez",
    "onze",
    "doze",
    "treze",
    "quatorze",
    "quinze",
    "dezesseis",
    "dezessete",
    "dezoito",
    "dezenove",
];
const centenas: [&str; 10] = [
    "",
    "cem",
    "duzendos",
    "trezentos",
    "quatrocentos",
    "quinhentos",
    "seiscentos",
    "setecentos",
    "oitocentos",
    "novecentos",
];
const singulares: [&str; 7] = [
    "centavo",
    "real",
    "mil",
    "milhão",
    "bilhão",
    "trilhão",
    "quatrilhão",
];
const plurais: [&str; 7] = [
    "centavos",
    "reais",
    "mil",
    "milhões",
    "bilhões",
    "trilhões",
    "quatrilhões",
];

fn main() {
    let valor = get_f64("Escreva um valor qualquer: ");

    let extenso = extenso_valor(valor);
}

fn extenso_valor(valor: f64) -> String {
    let grupos = separar_grupos(valor);
    let total = grupos.len();

    let mut partes = Vec::new();

    for (i, g) in grupos.iter().enumerate() {
        if *g == 0 {
            continue;
        }

        let ext = extenso_grupo(*g);
        let ordem = total - i - 1;

        let nome = if *g == 1 {
            singulares[ordem]
        } else {
            plurais[ordem]
        };

        partes.push(format!("{} {}", ext, nome));
    }

    if partes.is_empty() {
        return "Zero reais".to_string();
    }

    let texto = match partes.len() {
        1 => partes[0].clone(),
        _ => {
            let last = partes.last().unwrap().clone();
            let mut all = partes[..partes.len() - 1].join(", ");
            all.push_str(" e ");
            all.push_str(&last);

            all
        }
    };

    texto.to_uppercase()
}

fn separar_grupos(valor: f64) -> Vec<u32> {
    let centavos = (valor * 100_f64).round() as u64;
    let mut grupos = Vec::new();
    let mut n = centavos;

    while n > 0 {
        grupos.push((n % 1000) as u32);
        n /= 1000;
    }

    if grupos.is_empty() {
        grupos.push(0);
    }

    grupos.reverse();

    grupos
}

fn extenso_grupo(n: u32) -> String {
    let u = n % 10;
    let d = (n % 100) / 10;
    let c = n / 100;

    if n == 0 {
        return "".to_string();
    }

    if n > 100 && n < 200 {
        let resto = n % 100;
        if resto < 20 && resto >= 10 {
            return format!("cento e {}", dezenas_unit[resto as usize - 10]);
        } else if resto < 10 {
            return format!("cento e {}", unidades[resto as usize]);
        } else {
            return format!(
                "cento e {}",
                format!(
                    "{}{}",
                    dezenas[d as usize],
                    if u > 0 {
                        format!(" e {}", unidades[u as usize])
                    } else {
                        "".to_string()
                    }
                )
            );
        }
    }

    let mut partes = Vec::new();

    if c > 0 {
        partes.push(centenas[c as usize]);
    }

    if d == 1 {
        partes.push(dezenas_unit[u as usize]);
    } else {
        if d > 0 {
            partes.push(dezenas[d as usize]);
        }
        if u > 0 {
            partes.push(unidades[u as usize]);
        }
    }

    partes.join(" e ")
}
