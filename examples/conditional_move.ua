section .data
value_a dq 42
value_b dq 17
condition dq 1
result dq 0

section .text
global _start
_start:
    ; Load values
    load r0, [value_a]     ; r0 = 42
    load r1, [value_b]     ; r1 = 17
    load r2, [condition]   ; r2 = condition

    ; Test conditional moves based on comparison
    cmp r0, r1             ; compare value_a with value_b

    ; Test various conditional moves
    mov r3, 100            ; default value
    cmovgt r3, r0          ; if value_a > value_b, r3 = value_a
    store [result], r3     ; should store 42

    ; Test equal condition
    cmp r2, 1              ; check if condition == 1
    mov r4, 999
    cmoveq r4, r1          ; if condition == 1, r4 = value_b

    ; Test not equal
    cmp r2, 0
    mov r5, 555
    cmovne r5, r0          ; if condition != 0, r5 = value_a

    ; Test less than
    mov r6, 10
    cmp r1, r0             ; compare value_b with value_a
    cmovlt r6, r1          ; if value_b < value_a, r6 = value_b

    ; Test greater or equal
    mov r7, 20
    cmp r0, r1
    cmovge r7, r0          ; if value_a >= value_b, r7 = value_a

    ; Test overflow and sign conditions
    mov r8, 0
    test r0, r0            ; test if value_a is positive
    cmovns r8, r0          ; if not sign (positive), r8 = value_a

    ; Exit with final result
    mov r0, r8
    syscall exit
