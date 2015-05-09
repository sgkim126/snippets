// from vim(os_unix.c)

/*
 * Find out if the stack grows upwards or downwards.
 * "p" points to a variable on the stack of the caller.
 */
    static void
check_stack_growth(p)
    char        *p;
{
    int         i;

    stack_grows_downwards = (p > (char *)&i);
}
