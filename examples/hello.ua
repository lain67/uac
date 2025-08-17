section .data
msg db "Hello, World!", 0xA, 0
msg_len equ 14

section .text
global _start
_start:
    ; write syscall
    mov r0, 1          ; stdout
    lea r1, [msg]      ; message address
    mov r2, msg_len    ; message length
    syscall write
    
    ; exit syscall
    mov r0, 0          ; exit code
    syscall exit