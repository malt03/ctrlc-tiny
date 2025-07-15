#ifndef SIGINT_H
#define SIGINT_H

#include <signal.h>

int init_sigint_handler(void);
sig_atomic_t get_is_sigint_received(void);
void reset_is_sigint_received(void);

#endif
