#include <signal.h>
#include <stdatomic.h>
#include <stddef.h>

volatile sig_atomic_t is_sigint_received = 0;

void handle_sigint(int signo)
{
    (void)signo;
    is_sigint_received = 1;
}

int init_sigint_handler()
{
    struct sigaction sa = {0};
    sa.sa_handler = handle_sigint;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_RESTART;
    return sigaction(SIGINT, &sa, NULL);
}

sig_atomic_t get_is_sigint_received()
{
    return is_sigint_received;
}

void reset_is_sigint_received()
{
    is_sigint_received = 0;
}
