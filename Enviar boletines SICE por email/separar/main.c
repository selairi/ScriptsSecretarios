/** Este programa separa las páginas de un pdf en archivos diferentes.
 */


#include <poppler.h>
#include <cairo.h>
#include <cairo-pdf.h>
#include <pango/pangocairo.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>



gchar *get_uri(const char *filename) {
    GError *error = NULL;
    gchar *absolute, *uri;

    if (g_path_is_absolute(filename)) {
        absolute = g_strdup (filename);
    } else {
        gchar *dir = g_get_current_dir ();
        absolute = g_build_filename (dir, filename, (gchar *) 0);
        free (dir);
    }

    uri = g_filename_to_uri (absolute, NULL, &error);
    free (absolute);
    if (uri == NULL) {
        printf("poppler fail: %s\n", error->message);
        return NULL;
    }

    return uri;
}



int main(int argn, char *argv[])
{
    GError *error;
    cairo_status_t status;
    double width, height;
    cairo_surface_t *surface;
    cairo_t *cr;
    char buffer[100];

    // Se toma el primer argumento como fichero a procesar
    if(argn < 2) {
    	printf("Se necesita un argumento como nombre de archivo\n");
    	return 0;
    }

    gchar *uri = get_uri(argv[1]);

    PopplerDocument *document = poppler_document_new_from_file (uri, NULL, &error);
    if (document == NULL) {
        printf("poppler fail: %s\n", error->message);
        return 1;
    }

    int num_pages = poppler_document_get_n_pages (document);

    // Bucle de dibujo de páginas
    for (int i = 0; i < num_pages; i++) {
        PopplerPage *page = poppler_document_get_page (document, i);
        if (page == NULL) {
            printf("poppler fail: page not found\n");
            return 1;
        }
        poppler_page_get_size (page, &width, &height);

        sprintf(buffer, "%02d.pdf", i+1);

        surface = cairo_pdf_surface_create (buffer, 842, 595);
        cr = cairo_create (surface);

        cairo_pdf_surface_set_size (surface, width, height);


        poppler_page_render (page, cr);


        cairo_destroy (cr);
        cairo_surface_finish (surface);
        status = cairo_surface_status(surface);
        if (status)
            printf("%s\n", cairo_status_to_string (status));
        cairo_surface_destroy (surface);
    }
}