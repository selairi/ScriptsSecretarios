use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::fmt::Debug;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Tipos {
	ENTRY, TEXT, COMBOBOX, CHECKBOX, OPTIONS, EMAIL, SECTION, LABEL, DATE, IMAGE, FILE, REORDER
}

enum UltimoCampo {
	NONE, ETIQUETAWEB, ETIQUETAPDF, VALOR, OBSERVACIONES, CABECERA, LATERAL, PIE, PDF, MENSAJEACEPTADO
}

#[derive(Debug)]
pub struct Propiedad {
    pub valor: String,
    pub vacio: bool
}

impl Propiedad {
    pub fn new() -> Propiedad {
        Propiedad {
            valor: "".to_string(),
            vacio: true
        }
    }
}

#[derive(Debug)]
pub struct EntradaFormulario {
	pub id: String,
	pub etiqueta_web: String,
	pub etiqueta_pdf: Propiedad,
	pub valor: String,
	pub obligatorio: bool,
	pub observaciones: String,
	pub tipo: Tipos
}

#[derive(Debug)]
pub struct Formulario {
	pub titulo: String,
	pub texto_cabecera: String,
	pub texto_lateral: String,
	pub texto_pie: String,
    pub mensaje_formulario_aceptado: Propiedad,
	pub pdf: Propiedad,
	pub entradas: Vec::<EntradaFormulario>
}

impl EntradaFormulario {
	pub fn new() -> EntradaFormulario {
		EntradaFormulario {
			id: "".to_string(),
			etiqueta_web: "".to_string(),
			etiqueta_pdf: Propiedad::new(),
			obligatorio: false,
			observaciones: "".to_string(),
			valor: "".to_string(),
			tipo: Tipos::ENTRY
		}
	}
}


fn read_file_and_run_include(filename: &str) -> String {
    println!("Leyendo archivo {}", filename);
    let file = File::open(filename).expect("No se puede leer el archivo");
	let mut reader = io::BufReader::new(file);
	let mut line = String::new();
    let mut texto = String::new();
	while reader.read_line(&mut line).expect("Error leyendo el archivo") > 0 {
        println!("{}", line);
        if line.starts_with("Incluir:") {
            let archivo = line[8..].trim();
            println!("Archivo encontrado para incluir {}", archivo);
            texto += &read_file_and_run_include(archivo);
        } else if line.starts_with("#") {
            // Es un comentario, se ignora
        } else {
            texto += &line;
        }
        line.clear();
    }
    return texto;
}

pub fn read_file(filename: &str, ids: &mut HashSet<String>)-> Formulario {
	//let file = File::open(filename).expect("No se puede leer el archivo");

	let mut formulario = Formulario {
		titulo: "".to_string(),
		texto_cabecera: "".to_string(),
		texto_lateral: "".to_string(),
		texto_pie: "".to_string(),
        mensaje_formulario_aceptado: Propiedad::new(),
		pdf: Propiedad::new(),
		entradas: Vec::<EntradaFormulario>::new()
	};

    let archivo_texto = &read_file_and_run_include(filename);

	//let mut reader = io::BufReader::new(file);
	//let linea : String;
	let mut ultimo_campo = UltimoCampo::NONE;
	let mut entrada = EntradaFormulario::new();
	let mut n = 0;
	//while reader.read_line(&mut line).expect("Error leyendo el archivo") > 0 {
    for linea in archivo_texto.lines() {
        let line = format!("{}\n", &linea);
		println!("{} {}", n, line);
		n += 1;
	    if line.starts_with("Id:") {
	    	//println!("Id encontrado {}", line);
	    	if entrada.id.len() > 0 {
	    		if ids.contains(&entrada.id) {
		    		println!("Error: Identificador repetido: {}:{}: {}", filename, n, &entrada.id);
		    		std::process::exit(1);
		    	} else {
		    		ids.insert(entrada.id.to_string());
		    	}
	    		formulario.entradas.push(entrada);
	    		entrada = EntradaFormulario::new();
	    	}
	    	entrada.id = line[3..].trim().to_string();
	    	ultimo_campo = UltimoCampo::NONE;
	    } else if line.starts_with("EtiquetaWeb:") {
	    	//println!("EtiquetaWeb encontrado {}", line);
	    	entrada.etiqueta_web = line[12..].trim().to_string();
	    	ultimo_campo = UltimoCampo::ETIQUETAWEB;
	    } else if line.starts_with("EtiquetaPDF:") {
	    	//println!("etiqueta_pdf encontrado {}", line);
	    	entrada.etiqueta_pdf.valor = line[12..].trim_start().to_string();
            entrada.etiqueta_pdf.vacio = false;
	    	ultimo_campo = UltimoCampo::ETIQUETAPDF;
	    } else if line.starts_with("Observaciones:") {
	    	entrada.observaciones = line[14..].trim_start().to_string();
	    	ultimo_campo = UltimoCampo::OBSERVACIONES;
	    } else if line.starts_with("Valor:") {
	    	entrada.valor = line[6..].trim_start().to_string();
	    	if entrada.valor.len() == 0 {
	    		entrada.valor = "\n".to_string();
	    	}
	    	ultimo_campo = UltimoCampo::VALOR;
	    } /* else if line.starts_with("Incluir:") {
	    	if entrada.id.len() > 0 {
	    		formulario.entradas.push(entrada);
	    		entrada = EntradaFormulario::new();
	    	}
	    	let archivo = line[8..].trim();
	    	println!("Leyendo archivo >{}<", archivo);
	    	for e in read_file(archivo, ids).entradas {
	    		formulario.entradas.push(e);
	    	}
	    	ultimo_campo = UltimoCampo::NONE;
	    } */ else if line.starts_with("Tipo:") {
	    	entrada.tipo = match line[5..].trim() {
	    		"Texto" => Tipos::TEXT,
	    		"Desplegable" => Tipos::COMBOBOX,
	    		"Opciones" => Tipos::OPTIONS,
	    		"Email" => Tipos::EMAIL,
	    		"Seccion" => Tipos::SECTION,
	    		"Etiqueta" => Tipos::LABEL,
	    		"Checkbox" => Tipos::CHECKBOX,
	    		"Fecha" => Tipos::DATE,
	    		"Foto" => Tipos::IMAGE,
	    		"Archivo" => Tipos::FILE,
                "Reorden" => Tipos::REORDER,
	    		_ => Tipos::ENTRY
	    	};
	    	ultimo_campo = UltimoCampo::NONE;
	    } else if line.starts_with("Obligatorio:") {
	    	entrada.obligatorio = match line[12..].trim() {
	    		"No" => false,
	    		_ => true
	    	};
	    	ultimo_campo = UltimoCampo::NONE;
	    } else if line.starts_with("Titulo:") {
	    	formulario.titulo = line[7..].trim().to_string();
	    	ultimo_campo = UltimoCampo::NONE;
	    } else if line.starts_with("CabeceraPDF:") {
	    	formulario.texto_cabecera = line[12..].trim_start().to_string();
	    	ultimo_campo = UltimoCampo::CABECERA;
	    } else if line.starts_with("LateralPDF:") {
	    	formulario.texto_lateral = line[11..].trim_start().to_string();
	    	ultimo_campo = UltimoCampo::LATERAL;
	    } else if line.starts_with("PiePDF:") {
	    	formulario.texto_pie = line[7..].trim_start().to_string();
	    	ultimo_campo = UltimoCampo::PIE;
	    } else if line.starts_with("PDF:") {
	    	formulario.pdf.valor = line[4..].trim_start().to_string();
            formulario.pdf.vacio = false;
	    	ultimo_campo = UltimoCampo::PDF;
	    } else if line.starts_with("MensajeFormularioAceptado:") {
	    	formulario.mensaje_formulario_aceptado.valor = line[26..].trim_start().to_string();
            formulario.mensaje_formulario_aceptado.vacio = false;
	    	ultimo_campo = UltimoCampo::MENSAJEACEPTADO;
	    } else if line.starts_with("\t") || line.starts_with(" ") {
	    	match ultimo_campo {
	    		UltimoCampo::ETIQUETAWEB => {
	    			entrada.etiqueta_web += &line[1..];
	    		},
	    		UltimoCampo::ETIQUETAPDF => {
	    			entrada.etiqueta_pdf.valor += &line[1..];
	    		},
	    		UltimoCampo::VALOR => {
	    			entrada.valor += &line[1..];
	    		},
	    		UltimoCampo::OBSERVACIONES => {
	    			entrada.observaciones += &line[1..];
	    		},
	    		UltimoCampo::CABECERA => {
	    			formulario.texto_cabecera += &line[1..];
	    		},
	    		UltimoCampo::LATERAL => {
	    			formulario.texto_lateral += &line[1..];
	    		},
	    		UltimoCampo::PIE => {
	    			formulario.texto_pie += &line[1..];
	    		},
	    		UltimoCampo::PDF => {
	    			formulario.pdf.valor += &line[1..];
	    		},
                UltimoCampo::MENSAJEACEPTADO => {
                    formulario.mensaje_formulario_aceptado.valor += &line[1..];
                },
	    		UltimoCampo::NONE => {}
	    	}
	    }
	}

	if entrada.id.len() > 0 {
		if ids.contains(&entrada.id) {
    		println!("Error: Identificador repetido: {}:{}: {}", filename, n, &entrada.id);
    		std::process::exit(1);
    	} else {
    		ids.insert(entrada.id.to_string());
    	}
		formulario.entradas.push(entrada);
	}

    return formulario;
}
