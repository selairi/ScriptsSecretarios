mod parse_config;
mod html;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;



enum ARGS {
	Ninguna 
}

fn main() {
	let mut ejecutable: String = "".to_string();
	let mut opcion = ARGS::Ninguna;
    let mut archivos = Vec::new();
	for arg in std::env::args() {
		if ejecutable.len() == 0 {
			ejecutable = arg;
		}/* else if arg == "--public_key" {
			opcion = ARGS::PublicKey;
		} else if arg == "--private_key" {
			opcion = ARGS::PrivateKey;
		}*/ else {
			match opcion {
				ARGS::Ninguna => archivos.push(arg),
			}
			opcion = ARGS::Ninguna;
		}
	}
	if archivos.len() == 0 {
		println!(r###"{} [opciones] archivo
		"###, ejecutable);
		return;
	}
    for archivo in archivos {
    	println!("Leyendo el archivo {}", archivo);
    	let mut ids = HashSet::<String>::new();
       let formulario = parse_config::read_file(&archivo, &mut ids);
       let salida = html::html(&formulario);
    
       // Se escribe el html en un archivo
       let titulo = match formulario.titulo.len() {
    		0 => "index",
    		_ => &formulario.titulo
       };
       let file = File::create(format!("{}.html", titulo)).expect("Error al crear el archivo html");
       let mut out = io::BufWriter::new(file);
    
       out.write(salida.as_bytes()).unwrap();
    
       println!("{}", salida);
    }

}
