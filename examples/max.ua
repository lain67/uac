section .data
numbers dq 3, 7, 1, 9, 4, 2, 8, 5
array_size equ 8

section .text
global _start
_start:
    ; Initialize
    lea r1, [numbers]  ; array pointer
    load r0, [r1]      ; max = first element
    mov r2, 1          ; index = 1
    
find_max_loop:
    ; Check if we've processed all elements
    cmp r2, array_size
    jge done
    
    ; Load next element
    mov r3, r2
    mul r3, 8          ; multiply index by 8 (size of dq)
    add r3, r1         ; add to base address
    load r4, [r3]      ; load array[index]
    
    ; Compare with current max
    cmp r4, r0
    jle skip_update
    mov r0, r4         ; new max found
    
skip_update:
    inc r2             ; index++
    jmp find_max_loop
    
done:
    ; Exit with maximum value
    syscall exit