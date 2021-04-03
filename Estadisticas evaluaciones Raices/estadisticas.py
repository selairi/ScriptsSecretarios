import os
import glob
import csv

NOTAS = ['NE', 'IN', 'SU', 'BI', 'NT', 'SB', 'MH', ' 1', ' 2', '3', '4', '5', '6', '7', '8', '9', '10', 'APTO']
APROBADO = ['5', '6', '7', '8', '9', '10', 'APTO', 'SU 5', 'BI 6', 'NT 7','NT 8', 'SB 9', 'SB 10','MH']
SUSPENSO = ['NE', 'IN 1', 'IN 2', 'IN 3', 'IN 4', '1', '2', '3', '4']

def contiene(texto, lista):
    for l in lista:
        if l in texto:
            return True
    return False

def es_igual(texto, lista):
    for l in lista:
        if l == texto:
            return True
    return False

class Alumno:
    def __init__(self, nombre, n_expediente):
        self.nombre = nombre
        self.materias = []
        self.unidad = None
        self.n_expediente = n_expediente

    def toTexto(self):
        texto = ''
        for m in self.materias:
            texto += '{0} {1} {2}\n'.format(self.unidad, self.nombre, m.toTexto())
        return texto

class Materia:
    def __init__(self, materia, nota):
        self.materia = materia
        self.nota = nota

    def toTexto(self):
        return '{0}\t{1}'.format(self.materia, self.nota)

    def aprobada(self):
        if es_igual(self.nota, APROBADO):
            return True
        return False

    def suspensa(self):
        if es_igual(self.nota, SUSPENSO):
            return True
        return False

class MateriaEstadistica:
    def __init__(self, materia):
        self.materia = materia
        self.aprobados = 0
        self.suspensos = 0
        self.n_alumnos = 0

    def suspenso(self):
        self.n_alumnos += 1
        self.suspensos += 1

    def aprobado(self):
        self.n_alumnos += 1
        self.aprobados += 1



class Procesar:
    def __init__(self):
        self.archivo = None
        self.alumnos = {}

    def setArchivo(self, archivo):
        self.archivo = archivo

    def procesar_boletines(self):
        materias_encontradas = False
        alumno = None
        unidad = None
        n_expediente = None
        materia = None
        #fin = os.popen("pdftotext -layout -f {pagina} -l {pagina} -x 20 -y 50 -W 800 -H 450 '{documento}' -".format(pagina = n_pagina, documento = self.archivo), "r")
        os.system("pdftotext -layout -x 20 -y 50 -W 800 -H 450 '{documento}'".format(documento = self.archivo))
        fin = os.popen("pdftotext -layout -x 20 -y 50 -W 800 -H 450 '{documento}' -".format(documento = self.archivo), "r")
        for linea in fin:
            l = linea.strip()
            print(l)
            if l.startswith("ALUMNO/A:"):
                alumno = l[len("ALUMNO/A:"):].strip()
                #print(alumno)
            elif l.startswith("UNIDAD:"):
                n = len("UNIDAD:")
                unidad = l[n:n+10].strip()
            elif l.startswith("NUMERO EXP:"):
                n_expediente = l[len("NUMERO EXP:"):].strip()
            elif l.startswith("MATERIAS"):
                materias_encontradas = True
            elif materias_encontradas:
                if len(l) > 0 and alumno != None:
                    if not l.startswith('BOLETÍN DE CALIFICACIONES'):
                        # Se trata de determinar si es una materia:
                        # En secundaria las materias tienen una nota del tipo IN, SU, BI, NT, SB, NE
                        if contiene(l, NOTAS):
                            # Es una materia de secundaria
                            if '   ' in l:
                                indice = l.index('   ')
                                materia = l[:indice].strip()
                                nota = l[indice:].replace('AC', '').replace('RE', '').strip()
                        if not alumno in self.alumnos.keys():
                            self.alumnos[alumno] = Alumno(alumno, n_expediente)
                            self.alumnos[alumno].unidad = unidad
                        if materia != None and nota != None:
                            self.alumnos[alumno].materias.append(Materia(materia, nota))
                            materia = None
                            nota = None
                else:
                    materias_encontradas = False
        fin.close()


# Calcula los aprobados y suspensos a partir de los alumnos de una unidad
def calcular_aprobados_suspensos(alumnos):
    materias = {}
    for alumno in alumnos:
        for m in alumno.materias:
            if not m.materia in materias.keys():
                materias[m.materia] = MateriaEstadistica(m.materia)
            if m.aprobada():
                materias[m.materia].aprobado()
            elif m.suspensa():
                materias[m.materia].suspenso()
            else:
                print('Error: No se encuentra la calificación de la materia')
                print(alumno.nombre, alumno.unidad, m.toTexto())
                exit(1)
    return materias

# Calcula el número de alumnos que suspenden un número de asignaturas
def calcular_n_suspensos(alumnos):
    # Se buscan el número máximo de asignaturas
    n_asignaturas = 0
    for alumno in alumnos:
        if n_asignaturas < len(alumno.materias):
            n_asignaturas = len(alumno.materias)
    suspenden = []
    for n in range(0, n_asignaturas + 1):
        suspenden.append(0)
    for alumno in alumnos:
        n_suspensas = 0
        for m in alumno.materias:
            if m.suspensa():
                n_suspensas += 1
        suspenden[n_suspensas] += 1
    return suspenden


# Se leen los alumnos de los boletines
procesar = Procesar()
for archivo in glob.glob('boletines/*.pdf'):
    procesar.setArchivo(archivo)
    procesar.procesar_boletines()

# Se muestran por pantalla
for alumno in procesar.alumnos.values():
    print(alumno.toTexto())

# Se separan los alumnos por unidades
unidades = {}
for alumno in procesar.alumnos.values():
    if not alumno.unidad in unidades.keys():
        unidades[alumno.unidad] = []
    unidades[alumno.unidad].append(alumno)




def fprinf(fout, texto):
    fout.write(texto)
    fout.write('\n')

# Para cada unidad se calculan los aprobados y suspensos
fout = open('salida.txt', 'w')
estadisticas = {}
for unidad in sorted(unidades.keys()):
    print('Unidad:', unidad, 'Matriculados:', len(unidades[unidad]))
    estadisticas[unidad] = calcular_aprobados_suspensos(unidades[unidad])
    fprinf(fout, unidad)
    fprinf(fout, '\tMatri\tAprob\tSusp\t\tAprob\tSuspensos')
    for k in estadisticas[unidad].keys():
        m = estadisticas[unidad][k]
        fprinf(fout, '\t{1}\t{2}\t{3}\t\t{4}%\t{5}%\t{0}'.format(m.materia, m.n_alumnos, m.aprobados, m.suspensos, int(100.0*m.aprobados/m.n_alumnos), int(100.0*m.suspensos/m.n_alumnos)))
    suspenden = calcular_n_suspensos(unidades[unidad])
    fprinf(fout, '\nNúmero de alumnos con n asignaturas suspensas:')
    texto = ''
    for n in range(0, len(suspenden)):
        texto += '{0}\t'.format(n)
    fprinf(fout, texto)
    texto = ''
    for n in range(0, len(suspenden)):
        texto += '{0}\t'.format(suspenden[n])
    fprinf(fout, texto)
    fprinf(fout, 'Nº de alumnos: {0}'.format(len(unidades[unidad])))
    fprinf(fout, '')
fout.close()

def sumar(lista):
    n = 0
    for i in lista:
        n += i
    return n

# Se cargan las unidades para agrupar
fin = open("unidades.txt")
agrupaciones = []
for linea in fin:
    agrupaciones.append(linea.split(' '))
fin.close()

# Se calculan las estadísticas por grupos
fout = open('salida.txt', 'a')
fprinf(fout, '\n\nResultados por unidades:')
fprinf(fout, 'Unidades\tAprueban 0,1 ó 2\tSuspenden más de 3')
suspenden_grupo = {}
for grupo in agrupaciones:
    aprobados = 0
    suspensos = 0
    n_alumnos = 0
    fprinf(fout, '')
    for unidad in grupo[1:]:
        unidad = unidad.strip()
        if unidad in unidades.keys():
            suspenden = calcular_n_suspensos(unidades[unidad])
            u_aprobados = sumar(suspenden[0:3])
            u_suspensos = sumar(suspenden[3:])
            u_n_alumnos = sumar(suspenden[0:3]) + sumar(suspenden[3:])
            fprinf(fout, '{0}\t{1}%\t{2}%'.format(unidad, int(u_aprobados/u_n_alumnos*100.0), 100 - int(u_aprobados/u_n_alumnos*100.0)))
            aprobados += u_aprobados
            suspensos += u_suspensos
            n_alumnos += u_n_alumnos
    if n_alumnos > 0:
        suspenden_grupo[grupo[0]] = int(100.0*suspensos/n_alumnos)
fprinf(fout, '\n\nResultados totales por niveles:')
fprinf(fout, 'Nivel\tAprueban 0,1 ó 2\tSuspenden más de 3')
for nivel in suspenden_grupo.keys():
    fprinf(fout, '{0}\t{1}%\t{2}%'.format(nivel, 100 - suspenden_grupo[nivel], suspenden_grupo[nivel]))
fout.close()