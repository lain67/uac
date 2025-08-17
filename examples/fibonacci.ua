section .text
global _start

fibonacci:
    ; Input: r0 = n
    ; Output: r0 = fib(n)
    cmp r0, 2
    jl base_case       ; if n < 2, return n
    
    ; Save n
    mov r3, r0
    
    ; Calculate fib(n-1)
    dec r0
    call fibonacci
    mov r1, r0         ; r1 = fib(n-1)
    
    ; Calculate fib(n-2)
    mov r0, r3
    sub r0, 2
    call fibonacci
    
    ; Return fib(n-1) + fib(n-2)
    add r0, r1
    ret
    
base_case:
    ret                ; return n (already in r0)

_start:
    mov r0, 10         ; Calculate 10th Fibonacci number
    call fibonacci
    
    ; Exit with result
    syscall exit