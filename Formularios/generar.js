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
		if(campo.tipo === "Seccion") {
			// Es una sección
			this.print("");
			doc.setFont(this.fontFamily, "bold");
			doc.setFontSize(this.fontSize + 2);
			var etiqueta = campo.etiqueta_pdf.length > 0 ? campo.etiqueta_pdf : campo.etiqueta_web;
			linea += etiqueta;
			this.print(linea);
			doc.setFont(this.fontFamily, "normal");
		} else if(campo.tipo === "Etiqueta") {
			var etiqueta = campo.etiqueta_pdf.length > 0 ? campo.etiqueta_pdf : campo.etiqueta_web;
			linea += etiqueta;
			this.print("");
			this.print(linea);
		} else if(campo.tipo === "Foto") {
			var etiqueta = campo.etiqueta_pdf.length > 0 ? campo.etiqueta_pdf : campo.etiqueta_web;
			this.print(etiqueta + ": ");
			var canvas = document.getElementById("_canvas_" + campo.id);
			var valor = canvas.toDataURL("image/jpeg");
			doc.addImage(valor, 'JPEG', this.x, this.y, 16, 20);
			this.y += 20;
			this.print("");
		} else if(campo.tipo === "Archivo") {
			// No se incluyen los archivos en el PDF
		}else {
			var etiqueta = campo.etiqueta_pdf.length > 0 ? campo.etiqueta_pdf : campo.etiqueta_web;
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
	createImageBitmap(file.files[0]).then(function(img) {
		var canvas =  document.getElementById("_canvas_" + id)
		var ctx = canvas.getContext("2d");
		ctx.drawImage(img, 0, 0, 160, 200);
	});
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
	var ok = true;
	var last_focus = null;
	for(var i=0; i < checkEntryList.length; i++) {
		var id = checkEntryList[i];
		var e = document.forms["formulario"][id];
		var etiqueta = document.getElementById("_etiqueta_" +id);
		if(e.value.trim().length == 0 || e.type === 'file' && e.files.length == 0) {
			ok = false;
			if(last_focus == null)
				last_focus = e;
			etiqueta.classList.add("w3-red");
		} else {
			etiqueta.classList.remove("w3-red");
		}
	}

	if(ok) {
		// Se puede generar el formulario
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
