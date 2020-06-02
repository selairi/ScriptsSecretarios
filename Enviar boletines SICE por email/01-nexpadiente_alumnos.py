import csv

fin = open('listado de alumnos-n expediente.txt')
fout = open('nexpediente.csv', 'w')
archivo_csv = csv.writer(fout)

grupo = ''
lineas_nia = False

for linea in fin:
	if linea.startswith('    Grupo'):
		grupo = linea[24:35].strip()
		print("Procesando", grupo)
		continue
	elif linea.startswith('Fecha'):
		lineas_nia = False
		continue
	elif linea.startswith('             Expediente                                            Alumno'):
		lineas_nia = True
		continue
	if lineas_nia:
		# Se tiene una línea de NIA
		nia = linea[16:34].strip()
		alumno = linea[38:].strip()
		if len(nia) > 0 and nia.isnumeric():
			print(grupo, nia, alumno)
			archivo_csv.writerow([grupo, nia, alumno])

fout.close()
fin.close()