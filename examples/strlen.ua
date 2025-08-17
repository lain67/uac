section .data
text db "Hello, UASM!", 0

section .text
global _start

string_length:
    ; Input: r0 = string address
    ; Output: r0 = length
    mov r1, r0         ; save original address
    mov r0, 0          ; length counter
    
strlen_loop:
    load r2, [r1]      ; load current byte
    cmp r2, 0          ; check for null terminator
    je strlen_done
    inc r0             ; increment length
    inc r1             ; next character
    jmp strlen_loop
    
strlen_done:
    ret

_start:
    lea r0, [text]     ; address of string
    call string_length
    
    ; Exit with string length
    syscall exit