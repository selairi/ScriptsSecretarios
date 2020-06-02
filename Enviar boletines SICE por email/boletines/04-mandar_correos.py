# Identificador del usuario del correo de EducaMadrid. Sólo el identificado. Por ejemplo:
# Si el correo es pepe@educa.madrid.org, se escribirá sólo "pepe".
usuario_correo = "pepe"

# Contraseña del correo de EducaMadrid
password_correo = "......"



################################################################################################

import glob
import os
import sys
import csv

import smtplib
from email.message import EmailMessage



def renombrar(archivo):
	fin = os.popen('pdftotext "{0}" -'.format(archivo))
	for linea in fin:
		linea = linea.strip()
		if linea.startswith('Alumno'):
			print(linea)
			nombre = linea[8:]
			print(nombre)
			os.system("mv '{0}' 'salida/{1}.pdf'".format(archivo, nombre))
			break
	fin.close()

def obtener_nombre_nexpediente_del_boletin(archivo):
	fin = os.popen('pdftotext "{0}" -'.format(archivo))
	nombre = ''
	nexpediente = -1
	for linea in fin:
		linea = linea.strip()
		if linea.startswith('Alumno'):
			print(linea)
			nombre = linea[8:].upper().strip()
			print(nombre)
		elif linea.startswith('Nº Exp:'):
			nexpediente = int(linea[7:])
	fin.close()
	return (nombre, nexpediente)

def extraer_hojas(archivo_boletines):
	os.system('rm salida/*.pdf')
	os.system('cp "{0}" salida'.format(archivo_boletines))
	os.system('cd salida ; ./pdfSepararPaginas "{0}"'.format(archivo_boletines))
	os.system('rm "salida/{0}"'.format(archivo_boletines))

	archivos = glob.glob('salida/*.pdf')

	for archivo in archivos:
		if archivo == archivo_boletines:
			print('-------------------- Ignorado ', archivo)
			continue
		renombrar(archivo)
		nombre = archivo.replace('.pdf', '')

	#os.system('cp "{0}" salida'.format(archivo_boletines))
	#os.system('cd salida ; zip -9r ../boletines.zip *.pdf')



def extraer_boletines(correos, usuario, password):
	os.system('rm boletines/*.zip')
	archivos = glob.glob('*.pdf')

	server = smtplib.SMTP(host='smtp.educa.madrid.org',port=25)

	server.login(usuario, password)

	for archivo in archivos:
		if not 'Bolet' in archivo:
			print('-------------------- Ignorado ', archivo)
			continue
		# Se busca el nombre del grupo
		grupo = archivo.split(' ')[0]
		print("Procesando ", grupo)
		extraer_hojas(archivo)
		# Se extrae el nombre del alumno
		pdfs = glob.glob('salida/*.pdf')
		for pdf in pdfs:
			nombre, nexpediente = obtener_nombre_nexpediente_del_boletin(pdf)
			print()
			print('--------------------------------------')
			print(nombre, nexpediente)
			if nexpediente in correos.keys():
				if len(correos[nexpediente][0]) > 0:
					print('Correo electrónico localizado', correos[nexpediente])
					msg = EmailMessage()
					msg.set_charset( 'utf-8' )
					msg['From'] = "secretaria.ies.villablanca.madrid@educa.madrid.org"
					msg['To'] = "{0}@educa.madrid.org".format(correos[nexpediente][0])
					#msg['To'] = "pedro.lucas@educa.madrid.org"
					print("{0}@educa.madrid.org".format(correos[nexpediente][0]))
					msg['Subject'] = "Boletín de notas {0}".format(nombre)
					msg.set_content('Se adjunta el boletín de notas de esta evaluación. Este mensaje ha sido generado automáticamente.')

					content_file = open(pdf, 'rb')
					content = content_file.read()
					content_file.close()
					msg.add_attachment(content, maintype='application/pdf', subtype='pdf', filename='{0}.pdf'.format(nombre))

					# Se manda el mensaje
					server.send_message(msg)
					print("Correo enviado")
					#server.quit()
					#exit(1)
				else:
					print('No se ha podido mandar el correo')
			print()

	server.quit()




def obtener_correos():
	correos = {}
	fin = open('nia-nexpediente.csv')
	leer_csv = csv.reader(fin)
	for row in leer_csv:
		id_correo = row[3]
		nombre = row[4]
		correos[int(row[1])] = [id_correo, nombre]
	fin.close()
	return correos

correos = obtener_correos()
extraer_boletines(correos, usuario_correo, password_correo)