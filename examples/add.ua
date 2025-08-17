section .data
num1 dq 42
num2 dq 58
result dq 0

section .text
global _start
_start:
    ; Load first number
    load r0, [num1]
    
    ; Load second number
    load r1, [num2]
    
    ; Add them
    add r0, r1
    
    ; Store result
    store [result], r0
    
    ; Exit with result as exit code
    syscall exit