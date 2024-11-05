/*
No rust, não existe o null como um tipo de dado
Em Rust, os dados são divididos em duas categorias, os tipos escalares, e os tipos compostos

Em escalares, temos inteiros, flutuantes, caracteres e outros
Em compostos temos tuplas e arrays

Pelo Rust ser uma linguagem estaticamente tipada, é necessário saber o tipo de dados das variáveis em tempo de compilação.
Em casos em que muitos tipos podem ser aplicados, como converter um texto para número por exemplo, usando um parse, precisaremos adicionar uma anotação de tipo

Um exemplo é: let guess: u32 = "42".parse().expect("Not a number!");

Um tipo escalar representa um valor único, como integers, floating-points, numbers e booleans

ESCALARES

INTEGERS
O tipo comumente utilizado será uLength, isso significa um valor não assinado (unsigned, por isso a letra u)
Quando um integer for um inteiro assinado, será usada a letra i, o tipo tem por padrão 32 bits de espaço

* Um número não assinado (sem uma assinatura/sinal) é considerado sempre positivo, por isso precisamos assinar (dar um sinal a) o número quando precisamos de um valor menor que zero
* Em caso de números assinados, o range sempre é metade do valor originalmente pensado, por exemplo, quando se tem 255 caracteres, você agora somente pode armazenar 127 positivos e 127 negativos, incluindo o "0", tem-se 255
* Unsigned não aceita o "0"

Tamanhos na memória:
Length      Signed      Unsigned
8-bit       i8          u8
16-bit      i16         u16
32-bit      i32         u32
64-bit      i64         u64
128-bit     i128        u128
arch        isize       usize

* Para saber qual a capacidade de cada tamanho:
Unsigned            2 ^ length -1
Signed (min/max)    -(2 ^ (length -1)) ou 2 ^ (length -1) -1

FLOATING-POINTS
Os tipos de dados de ponto flutuante são expressos em Rust por f32 e f64, empregados para representar decimais e reais com precisão variável.
f32 > denota pontos flutuantes simples (32bits)
f64 > denota pontos flutuantes de precisão dupla (64bits)

Embora sejam cruciais para operações matemáticas de precisão decimal, os desenvolvedores devem se ater às limitações da representação binária de números de ponto flutuante. Assim, devem adotar estratégias adequadas para mitigar possíveis imprecisões nos cálculos

BOOLEANS
O Booleano em Rust pode ser representado por bool, true ou false. Ele representa estados verdadeiros ou falsos.
Normalmente são utilizados em estruturas condicionais, como instruções if e while, para controlar a execução do código com base em condições.
Além disso, os operadores como && (AND), || (OR) e ! (NOT), podem ser aplicados a valores booleanos para formar expressões lógicas mais complexas.
Em Rust, os booleanos são cruciais na escrita de código robusto e eficiente, contribuindo para a tomada de decisões a lógica de controle em diversas situações.

CHARACTERS
O Caractere em Rust é representado pelo tipo char, usado para armazenar um único Unicode. Diferente das linguagems que usam apenas bytes para representar caracteres, Rust usa o padrão Unicode, permitindo uma manipulação de uma gama maior de caracteres, incluindo de idiomas não ocidentais e outros símbolos especiais.
O tipo char é delimitado por aspas simples (''), e operações com caracteres podem ser realizadas para manipular e/ou comparar individualmente os elementos.
Rust também oferece funções para lidar com strings, que são coleções de caracteres, além de tratar a manipulação de caracteres como parte integrante do suporte a Unicode, o que falicita no desenvolvimento com textos multilíngues.

COMPOSTOS

TUPLAS
Uma Tupla possui um comprimento fixo, uma vez declaradas, não podem aumentar ou diminuir de tamanho
São como as listas, com valores separados por vírgulas, e são declaradas entre parênteses
Cada posição na Tupla possui um tipo, e os tipos dos diferentes valores na tupla não precisam ser iguais, por exemplo:

let tup: (i32, f64, u8) = (500, 6.4, 1);

ARRAYS
Todos os valores precisam ter o mesmo tipo quando se trata de um Array, e diferente de outras linguagens, os arrays em Rust também tem comprimento fixo.
Escrevemos os valores em matriz, como uma lista separada por vírgulas, declarada entre colchetes [], como no exemplo abaixo:

let a = [1, 2, 3, 4, 5];
*/

fn main() {
    println!("Hello, world!");
}
