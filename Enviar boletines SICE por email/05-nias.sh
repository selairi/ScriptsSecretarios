
# Se instalan las dependecias
sudo apt install libpoppler-glib-dev poppler-utils gcc make libgtk-3-dev python3

# Se compila el separador de páginas de pdf
# Este separador respeta los textos de cada página
cd separar
make
cd ..
cp separar/pdfSepararPaginas boletines/salida

# Se relaccionan los nº de expedientes con el NIA
pdftotext -layout listado\ de\ alumnos-n\ expediente.pdf
pdftotext -layout listado\ de\ alumnos-nia.pdf
python3 01-nexpadiente_alumnos.py
python3 02-nias_alumnos.py
python3 03-cruzar-nia-expediente.py
cp nia-nexpediente.csv boletines




