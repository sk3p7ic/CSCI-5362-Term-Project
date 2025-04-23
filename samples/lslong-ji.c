#include <grp.h>
#include <pwd.h>
#include <time.h>
#include <stdio.h>
#include <dirent.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <sys/stat.h>
#include <sys/types.h>


/**
 * Ensures that a given pointer `ptr` is not null. Prints `msg` and associated
 * error information, if any, and exits with `EXIT_FAILURE`.
 */
#define expect(ptr, msg) if (ptr == NULL) { perror(msg); exit(EXIT_FAILURE); }


/**
 * Converts a given UID `uid` to a C string of its name, if one exists.
 */
extern inline char *uidStr(short uid) {
    struct passwd *pw;
    if ((pw = getpwuid(uid)) == NULL)
        return "Unknown";
    return pw->pw_name;
}


/**
 * Converts a given GID `gid` to a C string of its name, if one exists.
 */
extern inline char *gidStr(short gid) {
    struct group *grp;
    if ((grp = getgrgid(gid)) == NULL)
        return "Unknown";
    return grp->gr_name;
}


/**
 * Writes the appropriate "rwx" values to a given buffer `str` having at least
 * 3 characters given some `permissionValue`.
 *
 * @ref https://www.redhat.com/en/blog/linux-file-permissions-explained
 */
extern inline void permbitsToChars(mode_t permissionValue, char *str) {
    // Check that string is long enough
    if (!(strlen(str) >= 3)) {
        fprintf(stderr, "Not enough characters in string (Expected >=3, Got %ld).", strlen(str));
        return;
    }
    if (permissionValue & 4)
        str[0] = 'r';
    if (permissionValue & 2)
        str[1] = 'w';
    if (permissionValue & 1)
        str[2] = 'x';
}


/**
 * For some `mode`, converts it to a C string representation of the file type
 * and permissions for the owner, group, and others.
 */
char *getFilemode(mode_t mode) {
    static char bits[11];
    strcpy(bits, "----------");

    // Get the type
    switch (mode & S_IFMT) {
        case S_IFREG: // Regular file
            bits[0] = '-';
            break;
        case S_IFDIR: // Directory
            bits[0] = 'd';
            break;
        case S_IFCHR: // Char I/O device
            bits[0] = 'c';
            break;
        case S_IFBLK: // Blk I/O device
            bits[0] = 'b';
            break;
        default: // Some other type (FIFO, socket, etc.)
            bits[0] = '?';
            break;
    }

    // Get the permission values for owner, group, and others
    permbitsToChars(mode >> 6, bits + 1);
    permbitsToChars(mode >> 3, bits + 4);
    permbitsToChars(mode     , bits + 7);

    return bits;
}


/**
 * Displays information about a file `fname` with file information `info` using
 * the same formatting found for entries in the shell command `ls -la`.
 *
 * @param fname The name of the file whose information is being printed.
 * @param info Information regarding the file which will be printed.
 */
void displayFileInfo(const char *fname, const struct stat *info) {
    printf("%s", getFilemode(info->st_mode));
    printf("%4d ", (int) info->st_nlink);
    printf("%-8s ", uidStr(info->st_uid));
    printf("%-8s ", gidStr(info->st_gid));
    printf("%8ld ", (long) info->st_size);
    printf("%.12s ", 4 + ctime(&info->st_mtime));
    printf("%s\n", fname);
}


/**
 * Used to encompass the result of the `statFile()` function.
 *
 * Stores the `stat` info about some file and a boolean value `ok` denoting
 * whether that `info` is safe to read.
 */
typedef struct {
    struct stat info;
    bool ok;
} StatFileResult;


/**
 * Retrieves information about a file if such a file exists.
 *
 * @param dirname The name of the directory (path) containing the file `fname`.
 * @param fname The name of the file to acquire information about, should it
 * exist.
 *
 * @returns The result of attempting to get the file info, including said info
 * if any exists and a boolean value of whether that information does exist.
 */
StatFileResult statFile(const char *dirname, const char *fname) {
    StatFileResult res;
    struct stat info;
    // If the file could not be stat'd
    if (stat(fname, &info) == -1) {
        // Attempt to stat using the full path
        size_t fnameWithPathSize = strlen(dirname) + 1 + strlen(fname) + 1;
        char *fnameWithPath = malloc(fnameWithPathSize);
        expect(fnameWithPath, "Could not allocate buffer to store full filename");
        // Construct the full path: <dirname>/<fname>
        snprintf(fnameWithPath, fnameWithPathSize, "%s/%s", dirname, fname);
        // If the file still cannot be stat'd (DNE or permission error)
        if (stat(fnameWithPath, &info) == -1) {
            perror(fname); // Report the error
            res.ok = false;
            free(fnameWithPath);
            return res;
        }
        free(fnameWithPath);
    }
    res.info = info;
    res.ok = true;
    return res;
}


/**
 * Displays a listing of files contained in `dirname` should that directory
 * exist.
 */
void displayDir(const char *dirname) {
    DIR *dir;
    struct dirent *dInfo;

    if ((dir = opendir(dirname)) == NULL) {
        fprintf(stderr, "Cannot open directory '%s': ", dirname);
        perror(""); // Hack to get strerr(errno)
        return;
    }
    // Read the files in the directory
    while ((dInfo = readdir(dir)) != NULL) {
        StatFileResult statRes = statFile(dirname, dInfo->d_name);
        if (statRes.ok)
            displayFileInfo(dInfo->d_name, &statRes.info);
    }
    closedir(dir);
}


int main(int argc, char **argv) {
    // If only the program name is given
    if (argc == 1) {
        displayDir(".");
        return EXIT_SUCCESS;
    }

    while (--argc) {
        printf("%s:\n", *(++argv));
        displayDir(*argv);
        // If there's still more directories to list, add a blank line
        if (argc - 1) printf("\n");
    }
}
