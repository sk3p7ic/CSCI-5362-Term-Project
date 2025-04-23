#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdbool.h>
#include <sys/wait.h>
#include <sys/types.h>


/**
 * Ensures that a given pointer `ptr` is not null. Prints `msg` and associated
 * error information, if any, and exits with `EXIT_FAILURE`.
 */
#define expect(ptr, msg) if (ptr == NULL) { perror(msg); exit(EXIT_FAILURE); }
/**
 * Calculates the size of a buffer, accounting for the null terminator (`\0`)
 */
#define buflen(strPtr) (strlen(strPtr) + 1)

#define PROMPT() printf("mysh%% ")
#define PROMPT_CMD(cmd) printf("mysh%% %s\n", cmd)


/**
 * Gets the next command from the user.
 * 
 * @param lastCommand A pointer to the last command ran (`NULL` if there is not
 * one). This is read from when the user attempts to run "!!" in the shell and
 * written to if the user attempts to run some command.
 * 
 * @returns The command C-string that the user would like to run, if any. This
 * is `NULL` otherwise.
 */
char *getNextCommand(const char *lastCommand) {
    size_t cmdBufSize = 16; // Initial size of the buffer
    char *cmdBuf = malloc(cmdBufSize);
    expect(cmdBuf, "Could not allocate buffer to read next command");

    PROMPT();

    // Get the input from the user
    // This is done char-by-char to allow for the dynamic buffer size
    size_t idx = 0;
    char c;
    while ((c = getc(stdin)) != '\n') {
        // If the input buffer needs to be bigger
        if (idx + 1 >= cmdBufSize) {
            cmdBufSize *= 2;
            cmdBuf = realloc(cmdBuf, cmdBufSize);
            expect(cmdBuf, "Could not reallocate buffer to read next command");
        }

        cmdBuf[idx++] = c; // Save this character to the buffer
    }
    cmdBuf[idx] = '\0'; // Ensure that the command string is null-terminated

    // If the user would like to run the last-run command
    if (strcmp(cmdBuf, "!!") == 0) {
        free(cmdBuf);
        if (lastCommand) { // Ensure that there is some previous command to load
            // Load this command into the buffer
            cmdBuf = malloc(buflen(lastCommand));
            expect(cmdBuf, "Could not allocate buffer to copy last command");
            strcpy(cmdBuf, lastCommand);
            PROMPT_CMD(cmdBuf);
        } else { // If there is no previous command
            printf("No commands in history.\n");
            cmdBuf = NULL;
        }
    }

    return cmdBuf;
}


/**
 * Gets the number of spaces found in a string (`char*`). This may be used to
 * give a rough estimate of size to pre-allocate an array of strings.
 */
size_t getNSpaces(const char *str) {
    size_t nSpaces = 0;
    while (*str) {
        if (*str++ == ' ') nSpaces++;
    }
    return nSpaces;
}


/**
 * Tokenizes a given string into an array of tokens (`char*` strings). Quoted
 * strings (excluding escaped ones) are parsed as a single string literal token.
 * 
 * @param str The input string to tokenize / parse.
 * @param strs The output array of tokens. This bufer must be pre-allocated with
 * enough size to fit each token pointer.
 * 
 * @returns The number of tokens written to `strs`.
 */
size_t tokenize(const char *str, char **strs) {
    char *toParse = malloc(buflen(str));
    expect(toParse, "Could not allocate buffer to parse string by spaces");
    strcpy(toParse, str);

    size_t idx = 0;

    char *curr = toParse;

    // Parse each string token, separated by spaces unless they are quoted
    // string literals
    while (*curr) {
        while (*curr == ' ') curr++; // Ignore leading spaces
        if (*curr == '\0') break; // If this is the end of the string

        char *token = curr; // Get a pointer to the start of this token
        if (*curr == '"') { // If there's a quoted string literal to parse
            token = ++curr;
            while (*curr && *curr != '"') curr++; // TODO: Handle escaped quotes
            if (*curr == '"') *curr++ = '\0'; // Null terminate for strcpy
        } else { // If this is a normal string token
            while (*curr && *curr != ' ') curr++;
            if (*curr) *curr++ = '\0'; // Null terminate for strcpy
        }

        strs[idx] = malloc(buflen(token));
        expect(strs[idx], "Could not allocate buffer for space-separated string token");
        strcpy(strs[idx++], token); // Save this token to the array
    }

    strs[idx++] = NULL; // Terminate the array
    if (toParse) free(toParse);
    return idx;
}


/**
 * Executes a given command. Supports asynchronously running commands.
 * 
 * @param nArgs The number of command arguments being passed in.
 * @param args The arguments to the command that will be run.
 */
void execute(size_t nArgs, char *args[]) {
    expect(args[0], "Command argument cannot be null");
    // Check if the user would like to run the command asynchronously
    bool doWait = strcmp(args[nArgs - 2], "&") != 0;
    if (!doWait) {
        args[nArgs - 2] = NULL;
    }

    pid_t pid, exitStatus;

    pid = fork();

    switch (pid) {
        case -1:
            expect(NULL, "Error running child process");
            break;
        case 0:
            execvp(args[0], args);
            expect(NULL, "Error");
            break;
        default:
            while (doWait && wait(&exitStatus) != pid);
    }

    // Free the arguments to prevent a leak
    for (size_t i = 0; i < nArgs; ++i) {
        if (args[i]) free(args[i]);
    }
    free(args);
}


int main(void) {
    char *cmdBuf, *lastCommand = NULL;

    while (1) {
        cmdBuf = getNextCommand(lastCommand);
        // Save this command to history if there is one, else restart loop
        if (cmdBuf && strlen(cmdBuf) > 0) {
            if (lastCommand) free(lastCommand);
            lastCommand = malloc(buflen(cmdBuf));
            expect(lastCommand, "Could not allocate buffer to store last command");
            strcpy(lastCommand, cmdBuf);
        } else {
            continue;
        }

        // If the user would like to exit
        if (strcmp(cmdBuf, "exit") == 0) {
            free(cmdBuf);
            if (lastCommand) free(lastCommand);
            exit(EXIT_SUCCESS);
        }

        // Get the number of arguments from the command:
        //  num spaces + 1 + 1 for null pointer to end the array for execvp
        size_t nArgs = getNSpaces(cmdBuf) + 1;
        char **cmdArgs = malloc((nArgs + 1) * sizeof(char*));
        // Tokenize the input to a list of arguments
        nArgs = tokenize(cmdBuf, cmdArgs);

        // Attempt to run the command
        execute(nArgs, cmdArgs);
        free(cmdBuf);
    }
}
