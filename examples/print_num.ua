section .data
number dq 12345
buffer resb 32
newline db 0xA

section .text
global _start

print_number:
    ; Input: r0 = number to print
    ; Convert number to string and print
    lea r1, [buffer]   ; buffer pointer
    add r1, 31         ; start from end of buffer
    store [r1], 0      ; null terminator
    dec r1
    
    mov r2, 10         ; divisor
    
convert_loop:
    xor r3, r3         ; clear remainder
    div r0, r2         ; divide by 10
    add r3, 48         ; convert remainder to ASCII
    store [r1], r3     ; store digit
    dec r1
    
    cmp r0, 0
    jne convert_loop
    
    inc r1             ; point to first digit
    
    ; Calculate string length
    lea r2, [buffer]
    add r2, 31
    sub r2, r1         ; length = end - start
    
    ; Print the number
    mov r0, 1          ; stdout
    ; r1 already points to string
    ; r2 already has length
    syscall write
    
    ; Print newline
    mov r0, 1
    lea r1, [newline]
    mov r2, 1
    syscall write
    
    ret

_start:
    load r0, [number]
    call print_number
    
    mov r0, 0
    syscall exit