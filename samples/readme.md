# C Code Samples

Please place C code samples in this directory. List the files and describe their purpose
here.


## Files

- `hello.c`: Contains no errors but rather serves as a test file for ChatGPT API queries.
- `race.c`: Contains C code with a race condition.
- `malloc-cpy.c`: Contains code which
  - Does not allocate a properly-sized buffer `buff` (should be `strlen(msg) + 1`).
  - Does not check that pointer `buff` was initialized properly and is not `NULL`.
  - Does not free the memory.
