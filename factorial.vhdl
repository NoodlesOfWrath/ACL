-- This is what the code should compile to
USE std.textio.ALL;

ENTITY factorial IS
END factorial;

ARCHITECTURE behaviour OF factorial IS
    FUNCTION factorial_fn(n : NATURAL) RETURN NATURAL IS
    BEGIN
        IF n = 0 THEN
            RETURN 1;
        ELSE
            RETURN n * factorial_fn(n - 1);
        END IF;
    END factorial_fn;
BEGIN
    PROCESS
        VARIABLE l : line;
    BEGIN
        write (l, STRING'("factorial of 5 is: "));
        write (l, INTEGER'(factorial_fn(5)));
        writeline (output, l);
        WAIT;
    END PROCESS;
END behaviour;