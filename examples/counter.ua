section .data
counter dq 0
limit dq 10

section .text
global _start
_start:
    ; Initialize counter
    mov r0, 0
    
count_loop:
    ; Store current counter value
    store [counter], r0
    
    ; Increment counter
    inc r0
    
    ; Check if we've reached the limit
    load r1, [limit]
    cmp r0, r1
    jl count_loop      ; if counter < limit, continue
    
    ; Exit with final counter value
    syscall exit