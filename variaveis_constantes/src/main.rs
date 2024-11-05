/*
A princípio, pode parecer que qualquer variável declarada sem o "mut" é uma constante devido ao comportamento ser parecido.
Entretanto, em Rust, há algumas diferenças entre as variáveis imutáveis e as constantes.

Exemplo:
let name = "Teste";
const NAME = &str "Teste2";

1 - As constantes são declaradas usando "const" ao invés de "let"
2 - As constantes devem sempre ser declaras com todas as letras maiúsculas
3 - Nas variáveis mutáveis e imutáveis, pode-se opcionalmente omitir o tipo delas (string, uN, fN, iN, tup), já nas constantes, é obrigatório a declaração do tipo
4 - As constantes são sempre transformadas pelo valor dentro delas, em todos os lugares, entretanto, isso é percebido ao analisar o binário gerado
    Exemplo: 
        let mut teste = "Apenas um teste"; > no binário, ele aparecerá como "teste" (nome da variável)
        const TESTE: &str = "Apenas outro teste"; > no binário, ele aparecerá como "Apenas outro teste" (valor da variável)

*/
fn main() {
    println!("Hello, world!");
}
