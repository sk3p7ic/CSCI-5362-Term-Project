#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    char msg[] = "Hello, world!";
    char *buff = malloc(strlen(msg));
    strcpy(buff, msg);
    printf("%s\n", buff);
}
