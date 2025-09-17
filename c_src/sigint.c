#include <signal.h>
#include <string.h>
#include <stdatomic.h>
#include <stddef.h>
#include <unistd.h>

volatile sig_atomic_t is_sigint_received = 0;
const char *sigint_message = NULL;
size_t sigint_message_size = 0;

void handle_sigint(int signo)
{
    (void)signo;
    is_sigint_received = 1;
    if (sigint_message)
    {
        write(STDERR_FILENO, sigint_message, sigint_message_size);
        write(STDERR_FILENO, "\n", 1);
    }
}

int init_sigint_handler(const char *message)
{
    if (message)
    {
        sigint_message = message;
        sigint_message_size = strlen(sigint_message);
    }
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
