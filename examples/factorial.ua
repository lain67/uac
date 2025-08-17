section .text
global _start

factorial:
    ; Input: r0 = n
    ; Output: r0 = n!
    mov r2, 1          ; result = 1
    
loop_start:
    cmp r0, 1
    jle loop_end       ; if n <= 1, done
    mul r2, r0         ; result *= n
    dec r0             ; n--
    jmp loop_start
    
loop_end:
    mov r0, r2         ; return result
    ret

_start:
    mov r0, 5          ; Calculate 5!
    call factorial
    
    ; Exit with factorial result
    syscall exit