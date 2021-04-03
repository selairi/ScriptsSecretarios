use crate::parse_config::*;

fn cabecera(formulario: &Formulario) -> String {

    let generar_js = r#####"
var _datos = {
	formulario: "",
	botonEnviar: null,
	canvasFoto:  null,
	inputFoto: null,
	inputFileFoto: null,
	campos: [],
	ultimoCampo: "",
	tagConInputId: null,
	datosIES: "",
	piePagina: "",
    mensaje_formulario_aceptado: "Se va a generar un archivo PDF que va a ser descargado.",
	cabeceraAddCurso: false,
	cabeceraEstandar: true,
	generarPDf: function() {}
};

_datos.dibujarBanderaComunidad = function (x, y) {
	// Se dibuja la bandera
	this.doc.setFont("zapfdingbats", "normal");
	this.doc.setFontSize(10);
	// Carácter de la estrella: H
	this.doc.rect(x, y, 16.5, 10);
	this.doc.text("H H H H", 1+x, 5 + y);
	this.doc.text("H H H", 3+x, 8 + y);
}

_datos.dibujarBanderaEuropa = function (x, y) {
	// Se dibuja la bandera
	this.doc.setFont("zapfdingbats", "normal");
	this.doc.setFontSize(6);
	// Carácter de la estrella: H
	this.doc.rect(x, y, 16.5, 10);
	for(var i=0.0; i<12.0; i+=1.0) {
		this.doc.text("H", x + 16.5/2.0 -1 + 4.0*Math.cos(i*Math.PI/6.0), y + 5 +1 + 4.0*Math.sin(i*Math.PI/6.0));
	}
}

_datos.cursoActual = function () {
	var d = new Date();
	var mes = d.getMonth() + 1;
	var agno = d.getFullYear();
	var curso = "Curso ";
	var agno_inicial = agno;
	var agno_final = agno + 1;
	if(mes < 6) {
		agno_inicial = agno - 1;
		agno_final = agno;
	}
	return curso + agno_inicial + "/" + agno_final;
}

_datos.cabeceraPDF = function (textoCabecera, textoLateral, numeroPagina) {
	var doc = this.doc;
	doc.setTextColor(100);
	doc.setDrawColor(100);
	this.dibujarBanderaComunidad(177, 10);
	//this.dibujarBanderaEuropa(177, 10);
	doc.setFont("helvetica", "bold");
	doc.setFontSize(10);
	this.printXY("Comunidad de Madrid", 160, 25);
	doc.setFont("helvetica", "bold");
	doc.setFontSize(12);
	if(this.cabeceraAddCurso)
		textoCabecera += this.cursoActual();
	this.printXY(textoCabecera, 62, 12, 95);
	doc.setFontSize(10);
	var pageHeight = doc.internal.pageSize.height;
	var pageWidth = doc.internal.pageSize.width;
	doc.setFont("helvetica", "normal");
	doc.text("Página " + numeroPagina, pageWidth - 50, pageHeight-7);
	doc.setFontSize(18);
	doc.setTextColor(0);
	doc.setDrawColor(0);
	//doc.text(textoLateral, 11, pageHeight - 50, {angle: 90});
	this.printXYRotado(textoLateral, 11, pageHeight - 50, 250, 90);
	doc.setTextColor(100);
	doc.setDrawColor(100);
	doc.setFont(this.fontFamily, this.fontStyle);
	doc.setFontSize(this.fontSize);
	if(!this.cabeceraEstandar) {
		doc.setFont("helvetica", "normal");
		doc.setFontSize(6);
		this.printXY(this.datosIES, 15, 12, 50);
	}
	this.y = this.margenSuperior;
	this.x = this.margenIzquierdo;
	doc.setFont(this.fontFamily, this.fontStyle);
	doc.setFontSize(this.fontSize);
	doc.setTextColor(0);
	doc.setDrawColor(0);
}

_datos.cabecera = function () {
	this.cabeceraPDF(this.textoCabecera, this.textoLateral, this.numeroPagina);
}

_datos.piePDF = function () {
	this.doc.setFontSize(this.fontSize - 2);
	this.print("");
	this.print(this.piePagina);
}

_datos.nuevaPagina = function () {
	var doc = this.doc;
	doc.addPage();
	this.numeroPagina++;
	this.y = this.margenSuperior;
}

_datos.print = function (texto) {
	var doc = this.doc;
	var pageHeight = doc.internal.pageSize.height;
	var pageWidth = doc.internal.pageSize.width;
	this.printXY(texto, this.x, this.y, pageWidth - this.margenIzquierdo - this.margenDerecho);
}

_datos.printXY = function (texto, x, y, anchura) {
	var doc = this.doc;
	this.y = y;
	this.x = x;
	var pageHeight = doc.internal.pageSize.height;
	var pageWidth = doc.internal.pageSize.width;
	var lines = doc.splitTextToSize(texto, anchura);
	for(var j=0; j<lines.length; j++){
		doc.text(lines[j], this.x, this.y);
		this.y += this.doc.getTextDimensions("Texto").h*1.1 + this.interlineado;
		if(this.y > (pageHeight - this.margenInferior)) {
			doc.addPage();
			this.numeroPagina++;
			this.cabeceraPDF(this.textoCabecera, this.textoLateral, this.numeroPagina);
			this.y = this.margenSuperior;
		}
	}
	this.x = this.margenIzquierdo;
}

_datos.printXYRotado = function (texto, x, y, anchura, angulo) {
	var doc = this.doc;
	this.y = y;
	this.x = x;
	var pageHeight = doc.internal.pageSize.height;
	var lines =doc.splitTextToSize(texto, anchura);
	for(var j=0; j<lines.length; j++){
		doc.text(lines[j], this.x, this.y, {angle: angulo});
		this.x += this.doc.getTextDimensions("Texto").h*1.1 + this.interlineado;
	}
	this.x = this.margenIzquierdo;
}


_datos.initPDF = function () {
	this.doc = new jsPDF({
		 orientation: 'p',
		 unit: 'mm',
		 format: 'a4',
		 putOnlyUsedFonts:true
	});

	var margenIzquierdo = margenDerecho = 20;
	var interlineado = 5;
	this.margenDerecho = margenDerecho;
	this.margenIzquierdo = margenIzquierdo;
	this.margenInferior = 30;
	this.margenSuperior = 30;
	this.interlineado = interlineado;
	this.numeroPagina = 1;
	this.y = this.margenSuperior;
	this.x = this.margenIzquierdo;
	this.textoCabecera = "IES Villablanca";
	this.textoLateral = "";
	this.fontSize = 10;
	this.fontFamily = "helvetica";
	this.fontStyle = "normal";
	this.doc.setFont(this.fontFamily, this.fontStyle);
	this.doc.setFontSize(this.fontSize);
	//this.interlineado = this.doc.getTextDimensions('Text').h * 1.3;
	this.interlineado = 0;
}


_datos.generarPDFEstandar = function () {
	var doc = this.doc;
	var pageHeight = doc.internal.pageSize.height;
	var pageWidth = doc.internal.pageSize.width;

	this.cabeceraPDF(this.textoCabecera, this.textoLateral, this.numeroPagina);
	doc.setFont(this.fontFamily, this.fontStyle);
	doc.setFontSize(this.fontSize);
	for(var i=0; i<items.length; i++) {
		var linea = '';
		var campo = items[i];
		doc.setFontSize(this.fontSize);
        var etiqueta = campo.etiqueta_pdf != null ? campo.etiqueta_pdf : campo.etiqueta_web;
        if( campo.etiqueta_pdf != null && campo.etiqueta_pdf.length == 0 ) {
		    etiqueta = null;
        }
        if(etiqueta == null) {
            // El campo no debe aparecer
        } else if(campo.tipo === "Seccion") {
			// Es una sección
			this.print("");
			doc.setFont(this.fontFamily, "bold");
			doc.setFontSize(this.fontSize + 2);
		    linea += etiqueta;
		    this.print(linea);
			doc.setFont(this.fontFamily, "normal");
		} else if(campo.tipo === "Etiqueta") {
			linea += etiqueta;
			this.print("");
			this.print(linea);
		} else if(campo.tipo === "Foto") {
			this.print(etiqueta + ": ");
			var canvas = document.getElementById("_canvas_" + campo.id);
			var valor = canvas.toDataURL("image/jpeg");
			doc.addImage(valor, 'JPEG', this.x, this.y, 16, 20);
			this.y += 20;
			this.print("");
		} else if(campo.tipo === "Archivo") {
			// No se incluyen los archivos en el PDF
		} else if(campo.tipo === "Checkbox") {
			var valor = document.forms["formulario"][campo.id].checked;
			this.print(etiqueta + ": " + (valor ? "Sí" : "No"));
		} else {
			var valor = document.forms["formulario"][campo.id].value.trim();
			this.print(etiqueta + ": " + valor);
		}
	}
	this.piePDF();
	doc.save('a4.pdf');
}

_datos.generarPDF = function (campos) {
	this.generarPDFEstandar();
}


function onCargarFoto(id) {
	//console.log("onCargarFoto");
	let file = document.getElementById(id);
	file.campoVacio = false;
	createImageBitmap(file.files[0]).then(function(img) {
		var canvas =  document.getElementById("_canvas_" + id)
		var ctx = canvas.getContext("2d");
		ctx.drawImage(img, 0, 0, 160, 200);
	});
}

function reorder_intercambiar_opcion(id_actual, id_anterior) {
    var valor_actual = document.forms["formulario"][id_actual];
    var valor_anterior = document.forms["formulario"][id_anterior];
    var aux = valor_actual.value;
    valor_actual.value = valor_anterior.value;
    valor_anterior.value = aux;
}

function reemplazar(texto, campos)
{
	for(var i=0; i<campos.length; i++) {
		var campo = campos[i];
		if(! (campo.tipo === "Seccion" || campo.tipo === "Etiqueta")) {
			var valor = document.forms["formulario"][campo.id].value.trim();
			texto = texto.replace(new RegExp('\\{' + campo.id + '\\}','g'), valor);
			texto = texto.replace(new RegExp('\\{curso_academico\\}','g'), _datos.cursoActual());
		}
	}
	return texto;
}



function generar()
{
	// Se verifica que todos los campos obligatorios hayan sido completados
	var ok = true;
	var last_focus = null;
	for(var i=0; i < checkEntryList.length; i++) {
		var id = checkEntryList[i];
		var e = document.forms["formulario"][id];
		var etiqueta = document.getElementById("_etiqueta_" +id);
		if(e.value.trim().length == 0 || e.type === 'file' && (e.campoVacio || e.files.length == 0)) {
			ok = false;
			if(last_focus == null)
				last_focus = e;
			etiqueta.classList.add("w3-red");
			if("classList" in e)
			 e.classList.add("w3-red");
		} else {
			etiqueta.classList.remove("w3-red");
			if("classList" in e)
			 e.classList.remove("w3-red");
		}
	}

	if(ok) {
		// Se puede generar el formulario
        alert(_datos.mensaje_formulario_aceptado);
		_datos.initPDF();
		_datos.textoCabecera = reemplazar(formulario.cabecera, items);
		_datos.textoLateral = reemplazar(formulario.lateral, items);
		_datos.piePagina = reemplazar(formulario.pie, items);
		var campos = {};
		for(var i=0; i<items.length; i++) {
			var campo = items[i];
			if(! (campo.tipo === "Seccion" || campo.tipo === "Etiqueta" || campo.tipo === "Foto")) {
				var valor = document.forms["formulario"][campo.id].value.trim();
				campos[campo.id] = valor;
			} else if(campo.tipo === "Foto") {
				//var file = document.getElementById(campo.id);
				var canvas =  document.getElementById("_canvas_" + campo.id);
				campos[campo.id] = canvas.toDataURL("image/jpeg");
				//file.value;
			}
		}
		_datos.generarPDF(campos);

	} else {
		alert("Hay campos sin completar o completados de forma errónea. Estos campos están marcados en rojo.");
		last_focus.focus();
	}

	return false;
}


"#####;


	return format!(r######"<!doctype html>
<head>
	<meta charset='utf8'/>
	<meta name="viewport" content="width=device-width, initial-scale=1">
	<link rel="stylesheet" href="https://www.w3schools.com/w3css/4/w3.css">
	<title>{titulo}</title>
</head>
<body>
<script src="https://cdnjs.cloudflare.com/ajax/libs/jspdf/1.5.3/jspdf.debug.js" integrity="sha384-NaWTHo/8YCBYJ59830LTz/P4aQZK1sS0SneOgAvhsIl3zBu8r9RevNg5lHCHAuQ/" crossorigin="anonymous"></script>
<script>{generar_js}</script>
<form name='formulario' onsubmit='return generar()' >
<h1 class='w3-blue'>{titulo}</h1>
<div class='w3-panel w3-border'>
"######, titulo = formulario.titulo, generar_js = generar_js).to_string();
}


fn pie(_formulario: &Formulario) -> String {
    let mut generar_zip : bool = false;
    for entrada in &_formulario.entradas {
        match entrada.tipo {
            Tipos::FILE => {
                generar_zip = true;
                break;
            },
            _ => {}
        }
    }
    let mut boton_zip : String = "".to_string();
    if generar_zip {
        boton_zip = "<span onclick='generarZip()' class='w3-btn w3-blue'>Generar Zip</span>".to_string();
    }
	return format!(r######"
</div>
<div class='boton'>
<span class="w3-btn w3-blue" onclick='generar()'>Generar documentación</span> {zip}
</div>
</form>
<script>
// Se resetean los campos file
{{
	for(var i=0; i < checkEntryList.length; i++) {{
		var id = checkEntryList[i];
		var e = document.forms["formulario"][id];
		var etiqueta = document.getElementById("_etiqueta_" +id);
		if(e.type === 'file') {{
			e.campoVacio = true;
		}}
	}}
}}
</script>
</body>
</html>
"######, zip = boton_zip).to_string();
}

fn escribir_entrada(entrada: &EntradaFormulario, input: &String) -> String {
	let obligatorio = match entrada.obligatorio {
		true => "*",
		false => ""
	};
	let salida = format!(r###"
		<div class='w3-panel w3-border w3-white'>
			<p><label id='_etiqueta_{id}'>{obligatorio} {texto}</label>:<br/> {input} <br/>
			{observaciones}</p>
		</div>
	"###, id = entrada.id, texto = entrada.etiqueta_web, observaciones = entrada.observaciones, obligatorio = obligatorio, input = input);
	return salida;
}

fn escribir_entry(entrada: &EntradaFormulario, tipo: &str) -> String {
	let salida = format!(r###"
		<input type='{tipo}' name='{id}' id='{id}' class='w3-input  w3-border' value='{valor}'/>
	"###, id = entrada.id, valor = entrada.valor, tipo = &tipo);
	return escribir_entrada(entrada, &salida);
}

fn escribir_checkbox(entrada: &EntradaFormulario) -> String {
	let salida = format!(r###"
		<input type='checkbox' name='{id}' id='{id}' class='w3-check'/>
	"###, id = entrada.id);
	return escribir_entrada(entrada, &salida);
}

fn escribir_combobox(entrada: &EntradaFormulario) -> String {
	let mut salida = format!(r###"
	 <select id="{id}" name="{id}">
	"###, id = entrada.id).to_string();
	for item in entrada.valor.lines() {
		salida += &format!("<option value='{valor}'>{valor}</option>", valor = item.trim());
	}
	salida += "</select>";
	return escribir_entrada(entrada, &salida);
}

fn escribir_options(entrada: &EntradaFormulario) -> String {
	let mut salida = "".to_string();
	for item in entrada.valor.lines() {
		salida += &format!("<input type='radio' value='{valor}' name='{id}'><label>{valor}</label> <br/>\n", valor = item.trim(), id = entrada.id);
	}
	return escribir_entrada(entrada, &salida);
}

fn escribir_text(entrada: &EntradaFormulario) -> String {
	let salida = format!(r###"
		<textarea name='{id}' id='{id}' class='w3-input w3-border'>{valor}</textarea>
	"###, id = entrada.id, valor = entrada.valor);
	return escribir_entrada(entrada, &salida);
}

fn escribir_section(entrada: &EntradaFormulario) -> String {
	let salida = format!(r###"
		</div>
		<div class='w3-panel w3-border w3-light-grey'>
			<h2>{texto}</h2>
	"###, texto = entrada.etiqueta_web);
	return salida.to_string();
}

fn escribir_label(entrada: &EntradaFormulario) -> String {
	let salida = format!(r###"
		<p>
			{texto}
		</p>
	"###, texto = entrada.etiqueta_web);
	return salida.to_string();
}

fn escribir_image(entrada: &EntradaFormulario) -> String {
	let salida = format!(r###"
		<input type='file' accept="image/*" onchange = 'onCargarFoto("{id}")'
			name='{id}' id='{id}' class='w3-input  w3-border' value='{valor}'/><br/>
		<canvas id='_canvas_{id}' width='160' height='200'></canvas>
	"###, id = entrada.id, valor = entrada.valor); //" Error en Vim al colorear sintaxis
	return escribir_entrada(entrada, &salida);
}

fn escribir_file(entrada: &EntradaFormulario) -> String {
	let salida = format!(r###"
		<input type='file' accept='{valor}'
			name='{id}' id='{id}' class='w3-input  w3-border'/><br/>
	"###, id = entrada.id, valor = entrada.valor.trim());
	return escribir_entrada(entrada, &salida);
}

fn escribir_reorder(entrada: &EntradaFormulario) -> String {
	let mut salida = "<table>".to_string();
    let mut n = 0;
    let n_valores = entrada.valor.lines().count() - 1;
	for item in entrada.valor.lines() {
        let boton_subir: String;
        if n > 0 {
            boton_subir = format!("<span class='w3-btn w3-indigo' onclick='reorder_intercambiar_opcion(\"{id}_{n}\", \"{id}_{anterior}\")'>Subir</span>", id = entrada.id, n = n, anterior =n - 1);
        } else {
            boton_subir = "".to_string();
        }
        let boton_bajar: String;
        if n < n_valores {
            boton_bajar = format!("<span class='w3-btn w3-green' onclick='reorder_intercambiar_opcion(\"{id}_{n}\", \"{id}_{siguiente}\")'>Bajar</span>", id = entrada.id, n = n, siguiente = n + 1);
        } else {
            boton_bajar = "".to_string();
        }
		salida += &format!(r###"<tr>
        <td>{boton_subir}</td> <td>{boton_bajar}</td>
        <td>{n1}º: <input type='text' value='{valor}' name='{id}_{n}' size='100' readonly ></td>
         </tr>
         "###, valor = item.trim(), id = entrada.id, n = n,
         boton_subir = boton_subir, boton_bajar = boton_bajar,
         n1 = n + 1);
        n += 1;
	}
    salida += "</table>";
	return escribir_entrada(entrada, &salida);

}


pub fn html(formulario: &Formulario) -> String {
	let mut salida: String = cabecera(formulario);

	for entrada in &formulario.entradas {
		match entrada.tipo {
			Tipos::ENTRY => {
				salida += &escribir_entry(entrada, "text");
			},
			Tipos::EMAIL => {
				salida += &escribir_entry(entrada, "email");
			},
			Tipos::DATE => {
				salida += &escribir_entry(entrada, "date");
			},
			Tipos::CHECKBOX => {
				salida += &escribir_checkbox(entrada);
			},
			Tipos::COMBOBOX => {
				salida += &escribir_combobox(entrada);
			},
			Tipos::OPTIONS => {
				salida += &escribir_options(entrada);
			},
			Tipos::TEXT => {
				salida += &escribir_text(entrada);
			},
			Tipos::IMAGE => {
				salida += &escribir_image(entrada);
			},
			Tipos::FILE => {
				salida += &escribir_file(entrada);
			},
			Tipos::SECTION => {
				salida += &escribir_section(entrada);
			},
			Tipos::LABEL => {
				if entrada.etiqueta_web.len() > 0 {
					salida += &escribir_label(entrada);
				}
			},
            Tipos::REORDER => {
                salida += &escribir_reorder(entrada);
            }
		};
	}

	// Se buscan los campos obligatorios
	salida += "<script>var checkEntryList = [";

	let mut primero = true;
	for entrada in &formulario.entradas {
		if entrada.obligatorio {
			let ok = match entrada.tipo {
				Tipos::ENTRY => true,
				Tipos::EMAIL => true,
				Tipos::IMAGE => true,
				Tipos::COMBOBOX => true,
				Tipos::OPTIONS => true,
				Tipos::DATE => true,
				Tipos::TEXT => true,
				Tipos::FILE => true,
				_ => false
			};
			if ok {
				if primero {
					primero = false;
				} else {
					salida += ",";
				}
				salida += &format!("\"{}\"", entrada.id)[0..];
			}
		}
	}

	salida += "];</script>\n";

	// Se genera la estructura con todos los campos
	salida += "<script>var items = [";
	let mut primero = true;
	for entrada in &formulario.entradas {
		if primero {
			primero = false;
		} else {
			salida += ",";
		}
        let pdf: String = match entrada.etiqueta_pdf.vacio {
            true => "null".to_string(),
            false => format!("\"{}\"", entrada.etiqueta_pdf.valor.trim().replace("\n", "\\n"))
        };
        match entrada.tipo {
            Tipos::REORDER => {
                // Si es del tipo "Reorden", se convierten en tipos entrada, una por cada valor:
                let mut n = 0;
                let max = entrada.valor.lines().count();
                while n < max {
                    let pdf_texto_reorden = match entrada.etiqueta_pdf.vacio {
                        true =>
                            if n == 0 {
                                format!("\"{}\\n   1º\"", entrada.etiqueta_web.trim().replace("\n", "\\n"))
                            } else {
                                format!("\"   {}º\"", n + 1)
                            },
                        false =>
                            if n == 0 {
                                format!("\"{}\n1º\"", entrada.etiqueta_pdf.valor.trim().replace("\n", "\\n"))
                            } else {
                                format!("\"{}º\"", n + 1)
                            }
                    };
                    salida += &format!(r###"
	    		        {{ id:'{id}_{n}', etiqueta_web: '{web}', etiqueta_pdf: {pdf}, observaciones: '{observaciones}', tipo: 'Entrada'
	    		        }}"###,
	    		        id = entrada.id, web = format!("{} {}º", entrada.etiqueta_web.trim().replace("\n", "\\n"), n + 1),
	    		        pdf = pdf_texto_reorden,
                        observaciones = entrada.observaciones.trim().replace("\n", "\\n"),
                        n = n);
                    n += 1;
                    if n < max {
                        salida += ",";
                    }
                }
            },
            _ => {
	    	    salida += &format!(r###"
	    		    {{ id:'{id}', etiqueta_web: '{web}', etiqueta_pdf: {pdf}, observaciones: '{observaciones}', tipo: '{tipo}'
	    		    }}"###,
	    		    id = entrada.id, web = entrada.etiqueta_web.trim().replace("\n", "\\n"),
	    		    pdf = pdf,
                    observaciones = entrada.observaciones.trim().replace("\n", "\\n"),
	    		    tipo = match entrada.tipo {
	    		    	Tipos::SECTION => "Seccion",
	    		    	Tipos::LABEL => "Etiqueta",
	    		    	Tipos::IMAGE => "Foto",
	    		    	Tipos::FILE => "Archivo",
	    		    	Tipos::CHECKBOX => "Checkbox",
	    		    	_ => "Entrada"
	    		    });
            }
        };
	}
	salida += "];</script>\n";

	// Se generan los datos del formulario
	salida += "<script>var formulario = {";
	salida += &format!("titulo: '{titulo}', cabecera: '{cabecera}', lateral: '{lateral}', pie: '{pie}'",
		titulo = formulario.titulo, cabecera = formulario.texto_cabecera.trim().replace("\n", "\\n"),
		lateral = formulario.texto_lateral.trim().replace("\n", "\\n"),
		pie = formulario.texto_pie.trim().replace("\n", "\\n"));
	salida += "};</script>\n";

	// Se genera el javascript del PDF
	if ! formulario.pdf.vacio {
		salida += &format!(r###"<script>_datos.generarPDF = function (campos) {{
			{pdf}
		}};</script>"###,
		pdf = formulario.pdf.valor);
	}

    // Se genera el javascript del PDF
	if ! formulario.mensaje_formulario_aceptado.vacio {
		salida += &format!(r###"<script>_datos.mensaje_formulario_aceptado = `{mensaje}`;</script>"###,
		mensaje = formulario.mensaje_formulario_aceptado.valor);
	}

/*
	// Se inicializan los canvas de firma
	salida += "<script>\n";
	for entrada in &formulario.entradas {
		match entrada.tipo {
			Tipos::SIGN => {},
			_ => {}
		}
	}
	salida += "</script>\n";
*/
	salida += &pie(formulario);

	return salida;
}
