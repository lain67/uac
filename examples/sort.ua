section .data
array dq 64, 34, 25, 12, 22, 11, 90
array_size equ 7

section .text
global _start

bubble_sort:
    ; Bubble sort implementation
    mov r0, 0          ; i = 0
    
outer_loop:
    cmp r0, array_size
    jge sort_done
    
    mov r1, 0          ; j = 0
    mov r2, array_size
    sub r2, r0         ; n - i
    dec r2             ; n - i - 1
    
inner_loop:
    cmp r1, r2
    jge next_outer
    
    ; Load array[j]
    lea r3, [array]
    mov r4, r1
    mul r4, 8          ; j * 8
    add r3, r4
    load r5, [r3]      ; array[j]
    
    ; Load array[j+1]
    add r3, 8
    load r6, [r3]      ; array[j+1]
    
    ; Compare
    cmp r5, r6
    jle no_swap
    
    ; Swap elements
    store [r3], r5     ; array[j+1] = array[j]
    sub r3, 8
    store [r3], r6     ; array[j] = array[j+1]
    
no_swap:
    inc r1             ; j++
    jmp inner_loop
    
next_outer:
    inc r0             ; i++
    jmp outer_loop
    
sort_done:
    ret

_start:
    call bubble_sort
    
    ; Exit successfully
    mov r0, 0
    syscall exit