use petgraph::dot::{Dot, Config};
use petgraph::graph::Graph;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut graph = Graph::<&str, &str>::new();

    // Adiciona nós
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");
    let g = graph.add_node("G");
    let h = graph.add_node("H");
    let i = graph.add_node("I");
    let j = graph.add_node("J");
    let k = graph.add_node("K");
    let l = graph.add_node("L");

    // Adiciona arestas
    graph.add_edge(a, b, "");
    graph.add_edge(b, c, "");
    graph.add_edge(c, d, "");
    graph.add_edge(d, a, ""); // ciclo

    graph.add_edge(a, e, "");
    graph.add_edge(e, f, "");
    graph.add_edge(f, g, "");
    graph.add_edge(g, h, "");
    graph.add_edge(h, e, ""); // segundo ciclo

    graph.add_edge(c, i, "");
    graph.add_edge(i, j, "");
    graph.add_edge(j, k, "");
    graph.add_edge(k, l, "");
    graph.add_edge(l, j, ""); // terceiro ciclo

    graph.add_edge(f, i, ""); // ligação entre regiões

    // Gera o conteúdo .dot
    let dot_output = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // Salva no arquivo
    let mut file = File::create("graph.dot")?;
    file.write_all(dot_output.as_bytes())?;

    println!("Arquivo 'graph.dot' gerado com sucesso!");
    Ok(())
}