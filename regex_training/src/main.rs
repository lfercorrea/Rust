use core::num;

use regex::{Captures, Regex};

fn main() {
    let re = Regex::new(r"\(\d{2}\)\s\d{5}-?\d{4}").unwrap();
    let text = "id=\"1pv8z9\"
        Ontem eu precisei ligar para várias pessoas e anotei os contatos assim:

        João: (14) 99876-1234
        Maria: (11) 912345678
        Carlos: (21)99999-0000
        Empresa: 0800-123-456
        Ana: (85) 98765-4321
        Pedro: (3) 91234-5678
        Julia: (47) 9987-1234
        Suporte: (31) 91234-ABCD
        Contato alternativo: (19) 93456-7890

        Também achei esses números antigos:
        (16) 3333-4444
        (22) 999999999
        (44) 91234-56789";

    for number in re.find_iter(text) {
        println!("Encontrado telefone: '{}'", number.as_str());
    }
}
