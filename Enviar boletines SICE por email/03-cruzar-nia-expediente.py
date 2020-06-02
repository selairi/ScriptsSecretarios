import csv

nias = {}
correos = {}

fin = open('nexpediente.csv')
fin1 = open('nias.csv')
fin2 = open('listadoUsuarios.csv')
fout = open('nia-nexpediente.csv', 'w')
salida_csv = csv.writer(fout)

nias_csv = csv.reader(fin1)
for row in nias_csv:
	nombre = row[2].upper()
	if nombre in nias.keys():
		print('Alumno en dos grupos', row[2])
	else:
		nias[nombre] = [row[0], row[1]]
	#print(row)

#print(nias)

correos_csv = csv.reader(fin2)
for row in correos_csv:
	if row[5].isnumeric():
		nia = int(row[5])
		id = row[3]
		correos[nia] = id

nexpediente_csv = csv.reader(fin)
for row in nexpediente_csv:
	#print(row)
	nombre = row[2].upper()
	nexpediente = row[1]
	grupo = row[0]
	nia = ''
	correo = ''
	if nombre in nias.keys():
		alumno = nias[nombre]
		#print(alumno, row)
		if not alumno[0] == row[0]:
			print('El grupo no coincide', row, alumno)
			# Se la añade con dos grupos
		nia = alumno[1]
		#salida_csv.writerow([row[0], row[1], alumno[1], nombre])
	else:
		print('No encontrado', row)
		# Posiblemente el nombre esté cortado en el expediente
		alternativas = []
		for nia_nombre in nias.keys():
			if nia_nombre.startswith(nombre):
				alternativas.append(nia_nombre)
		if len(alternativas) == 1:
			# Se ha encontrado el nia
			print('Alternativa encontrada', alternativas[0])
			nia = alumno[1]
			nombre = alternativas[0]
			#salida_csv.writerow([row[0], row[1], alumno[1], alternativas[0]])
		else:
			print('Varias alternativas', alternativas)
			continue
	# Se busca el correo
	if int(nia) in correos.keys():
		correo = correos[int(nia)]

	salida_csv.writerow([grupo, nexpediente, nia, correo, nombre])


fout.close()
fin1.close()
fin2.close()
fin.close()