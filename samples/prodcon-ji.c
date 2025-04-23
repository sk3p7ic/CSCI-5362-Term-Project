#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <semaphore.h>
#include <sys/semaphore.h>

#define BUF_SIZE 7

typedef int buf_item;

buf_item buffer[BUF_SIZE];
sem_t empty = BUF_SIZE, full = 0;
sem_t mutex;

void insertItem(buf_item item) {
    sem_wait(&mutex);
    sem_wait(&empty);
    buffer[full % BUF_SIZE] = item;
    sem_post(&full);
    sem_post(&mutex);
}

buf_item removeItem(void) {
    buf_item item = -1;
    sem_wait(&mutex);
    sem_wait(&full);
    item = buffer[full % BUF_SIZE];
    sem_post(&empty);
    sem_post(&mutex);
    return item;
}

int main(void) {
    printf("Hello, World!\n");
    sem_init(&empty, 0, BUF_SIZE);
    sem_init(&full, 0, 0);
    sem_init(&mutex, 0, 0);
}
