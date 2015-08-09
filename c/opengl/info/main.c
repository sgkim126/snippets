#include <stdio.h>

#include <GL/gl.h>
#include <GL/glu.h>
#include <GL/glut.h>

void print_opengl_information() {
    const GLubyte* version = glGetString(GL_VERSION);
    const GLubyte* vendor = glGetString(GL_VENDOR);
    const GLubyte* renderer = glGetString(GL_RENDERER);

    GLint max_stack_depth;
    GLint depth_bits;
    glGetIntegerv(GL_MAX_MODELVIEW_STACK_DEPTH, &max_stack_depth);
    glGetIntegerv(GL_DEPTH_BITS, &depth_bits);

    printf("Version: %s\n", version);
    printf("Vendor: %s\n", vendor);
    printf("Renderer: %s\n", renderer);
    printf("max stack depth: %d\n", max_stack_depth);
    printf("depth bits: %d\n", depth_bits);
}

int main(int argc, char** argv) {
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_DOUBLE | GLUT_RGBA);

    glutCreateWindow("Test");


    print_opengl_information();
}
