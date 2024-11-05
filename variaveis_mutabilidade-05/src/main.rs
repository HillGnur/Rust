/*
Variáveis em RUST

As variáveis são declaradas usando "let", o tipo de variável pode ser explicitamente indicado ou inferido pelo compilador.
A mutabilidade em Rust é explicitamente controlada, então por padrão, elas não são mutáveis, para torná-las mutáveis, utilizamos a palavra "mut" em seguida ao let
Mesmo que uma variável seja declarada de forma imutável, isso ainda não a tornará uma constante.

Imutável > let variavel = valor
Mutável > let mut variavel = valor
Constante > 

Além disso, o Rust trás o conceito de propriedade única para gerenciar a alocação de memória, onde cada valor tem um "proprietário", e quando o proprietário sai de escopo, o valor é liberado automaticamente
Exemplo:

let texto = String::from("hello"); //String estática, não podendo ser alterada
let texto2 = texto; //Ao declarar o valor de texto dentro de texto2, a variável texto será destruída, passando a propriedade e eliminando o que há antes (a nível de escopo)

* Para fazer referência a um valor sem destruir a variável que o contém, usamos o conceito de referência mutável ou imutável, passando o "&" para criar essa referência
Exemplo:

let mut valor = 50;
let referencia = &valor; //Imutável
let referencia_mut = &mut valor; //Mutável
* Por padrão, a referência não é mutável, ao usar &mut, a referência pode ir até o proprietário e pedir pra ele realizar a alteração
* Como o número 50 é um tipo primitivo, ele poderia ser copiado sem o "&" sem destruir o proprietário, entretanto, ela deixaria de ser uma referência sem o "&"

*/

fn main() {
    println!("Hello, world!");
}
