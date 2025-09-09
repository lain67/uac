section .data
test_val1 dq 42
test_val2 dq 17
test_val3 dq 42
zero_val dq 0
neg_val dq -5
pos_val dq 100
overflow_val dq 0x7FFFFFFFFFFFFFFF
large_val dq 0xFFFFFFFFFFFFFFFF
parity_test dq 0b11111111  ; even parity (8 ones)
no_parity_test dq 0b1111111 ; odd parity (7 ones)

; Result counters
je_count dq 0
jne_count dq 0
jl_count dq 0
jle_count dq 0
jg_count dq 0
jge_count dq 0
jo_count dq 0
jno_count dq 0
js_count dq 0
jns_count dq 0
jp_count dq 0
jnp_count dq 0
ja_count dq 0
jae_count dq 0
jb_count dq 0
jbe_count dq 0

section .text
global _start

_start:
    ; Test JE (Jump if Equal) and JNE (Jump if Not Equal)
    load r0, [test_val1]
    load r1, [test_val3]
    cmp r0, r1              ; Compare 42 with 42
    je equal_case           ; Should jump
    jmp ne_case

equal_case:
    load r2, [je_count]
    inc r2
    store [je_count], r2
    jmp test_not_equal

ne_case:
    load r2, [jne_count]
    inc r2
    store [jne_count], r2

test_not_equal:
    load r0, [test_val1]
    load r1, [test_val2]
    cmp r0, r1              ; Compare 42 with 17
    jne not_equal_case      ; Should jump
    jmp continue_tests

not_equal_case:
    load r2, [jne_count]
    inc r2
    store [jne_count], r2

continue_tests:
    ; Test JL (Jump if Less) and JG (Jump if Greater)
    load r0, [test_val2]
    load r1, [test_val1]
    cmp r0, r1              ; Compare 17 with 42
    jl less_case            ; Should jump
    jmp greater_test

less_case:
    load r2, [jl_count]
    inc r2
    store [jl_count], r2

greater_test:
    load r0, [test_val1]
    load r1, [test_val2]
    cmp r0, r1              ; Compare 42 with 17
    jg greater_case         ; Should jump
    jmp less_equal_test

greater_case:
    load r2, [jg_count]
    inc r2
    store [jg_count], r2

less_equal_test:
    ; Test JLE (Jump if Less or Equal)
    load r0, [test_val2]
    load r1, [test_val1]
    cmp r0, r1              ; Compare 17 with 42
    jle less_equal_case     ; Should jump
    jmp greater_equal_test

less_equal_case:
    load r2, [jle_count]
    inc r2
    store [jle_count], r2

greater_equal_test:
    ; Test JGE (Jump if Greater or Equal)
    load r0, [test_val1]
    load r1, [test_val3]
    cmp r0, r1              ; Compare 42 with 42 (equal)
    jge greater_equal_case  ; Should jump
    jmp sign_tests

greater_equal_case:
    load r2, [jge_count]
    inc r2
    store [jge_count], r2

sign_tests:
    ; Test JS (Jump if Sign) - negative
    load r0, [neg_val]
    test r0, r0             ; Test if negative
    js sign_case            ; Should jump (negative)
    jmp no_sign_test

sign_case:
    load r2, [js_count]
    inc r2
    store [js_count], r2

no_sign_test:
    ; Test JNS (Jump if No Sign) - positive
    load r0, [pos_val]
    test r0, r0             ; Test if positive
    jns no_sign_case        ; Should jump (positive)
    jmp overflow_tests

no_sign_case:
    load r2, [jns_count]
    inc r2
    store [jns_count], r2

overflow_tests:
    ; Test JO (Jump if Overflow) and JNO (Jump if No Overflow)
    ; Create an overflow condition
    load r0, [overflow_val]
    add r0, 1               ; This might overflow
    jo overflow_case        ; Jump if overflow occurred
    jmp no_overflow_case

overflow_case:
    load r2, [jo_count]
    inc r2
    store [jo_count], r2
    jmp parity_tests

no_overflow_case:
    load r2, [jno_count]
    inc r2
    store [jno_count], r2

parity_tests:
    ; Test JP (Jump if Parity) - even number of 1 bits
    load r0, [parity_test]
    test r0, r0             ; Set parity flag
    jp parity_case          ; Jump if even parity
    jmp no_parity_test_jump

parity_case:
    load r2, [jp_count]
    inc r2
    store [jp_count], r2

no_parity_test_jump:
    ; Test JNP (Jump if No Parity) - odd number of 1 bits
    load r0, [no_parity_test]
    test r0, r0             ; Set parity flag
    jnp no_parity_case      ; Jump if odd parity
    jmp unsigned_tests

no_parity_case:
    load r2, [jnp_count]
    inc r2
    store [jnp_count], r2

unsigned_tests:
    ; Test JA (Jump if Above) - unsigned greater than
    load r0, [pos_val]      ; 100
    load r1, [test_val1]    ; 42
    cmp r0, r1              ; Compare unsigned 100 with 42
    ja above_case           ; Should jump (100 > 42 unsigned)
    jmp above_equal_test

above_case:
    load r2, [ja_count]
    inc r2
    store [ja_count], r2

above_equal_test:
    ; Test JAE (Jump if Above or Equal) - unsigned greater or equal
    load r0, [test_val1]
    load r1, [test_val3]
    cmp r0, r1              ; Compare 42 with 42
    jae above_equal_case    ; Should jump (equal)
    jmp below_test

above_equal_case:
    load r2, [jae_count]
    inc r2
    store [jae_count], r2

below_test:
    ; Test JB (Jump if Below) - unsigned less than
    load r0, [test_val2]    ; 17
    load r1, [test_val1]    ; 42
    cmp r0, r1              ; Compare unsigned 17 with 42
    jb below_case           ; Should jump (17 < 42 unsigned)
    jmp below_equal_test

below_case:
    load r2, [jb_count]
    inc r2
    store [jb_count], r2

below_equal_test:
    ; Test JBE (Jump if Below or Equal) - unsigned less or equal
    load r0, [test_val2]    ; 17
    load r1, [test_val1]    ; 42
    cmp r0, r1              ; Compare unsigned 17 with 42
    jbe below_equal_case    ; Should jump (17 <= 42)
    jmp complex_conditions

below_equal_case:
    load r2, [jbe_count]
    inc r2
    store [jbe_count], r2

complex_conditions:
    ; Test complex condition chains
    load r0, [test_val1]    ; 42

    ; Chain: if (val >= 40 && val <= 50)
    cmp r0, 40
    jl skip_complex         ; if val < 40, skip
    cmp r0, 50
    jg skip_complex         ; if val > 50, skip

    ; Value is in range [40, 50]
    inc r0                  ; increment as success indicator

skip_complex:
    ; Test zero flag with different operations
    load r1, [zero_val]
    test r1, r1             ; Test for zero
    je zero_detected        ; Jump if zero
    jmp non_zero_path

zero_detected:
    mov r2, 999             ; Mark zero detection
    jmp boundary_tests

non_zero_path:
    mov r2, 111             ; Mark non-zero path

boundary_tests:
    ; Test boundary values for signed comparisons
    mov r3, 0x7FFFFFFFFFFFFFFF  ; Max positive 64-bit
    mov r4, 0x8000000000000000  ; Min negative 64-bit

    cmp r3, r4              ; Compare max pos with min neg
    jg max_greater          ; Should jump (max > min in signed)
    jmp final_tests

max_greater:
    mov r5, 1               ; Mark that max > min

final_tests:
    ; Test with immediate values
    mov r6, 10
    cmp r6, 5
    jg immediate_greater    ; Should jump
    jmp calculate_results

immediate_greater:
    mov r7, 1

calculate_results:
    ; Sum up all the test results
    load r0, [je_count]
    load r1, [jne_count]
    add r0, r1

    load r1, [jl_count]
    add r0, r1

    load r1, [jg_count]
    add r0, r1

    load r1, [jle_count]
    add r0, r1

    load r1, [jge_count]
    add r0, r1

    load r1, [js_count]
    add r0, r1

    load r1, [jns_count]
    add r0, r1

    ; Add other counters
    load r1, [ja_count]
    add r0, r1

    load r1, [jb_count]
    add r0, r1

    ; Ensure we have a reasonable result
    and r0, 0xFF            ; Keep result in valid exit code range

    ; Exit with total count of successful jumps
    syscall exit
