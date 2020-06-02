import csv

fin = open('listado de alumnos-nia.txt')
fout = open('nias.csv', 'w')
archivo_csv = csv.writer(fout)

grupo = ''
lineas_nia = False

for linea in fin:
	if linea.startswith('GRUPO:'):
		grupo = linea.split(' ')[1].strip()

		print("Procesando", grupo)
		continue
	elif linea.startswith('Fecha'):
		lineas_nia = False
		continue
	elif linea.startswith('  Nº       NIA       Apellidos, Nombre'):
		lineas_nia = True
		continue
	if lineas_nia:
		# Se tiene una línea de NIA
		nia = linea[5:20].strip()
		alumno = linea[20:].strip()
		if len(nia) > 0:
			print(grupo, nia, alumno)
			archivo_csv.writerow([grupo, nia, alumno])

fout.close()
fin.close()