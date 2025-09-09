section .data
test_value dq 0b1010110100110101
bit_mask dq 0xFF00FF00
result_bsf dq 0
result_bsr dq 0
result_bt dq 0

section .text
global _start
_start:
    ; Load test value
    load r0, [test_value]      ; r0 = test pattern

    ; Test bit scan forward (find first set bit)
    bsf r1, r0                 ; find position of first 1 bit
    store [result_bsf], r1     ; save result

    ; Test bit scan reverse (find last set bit)
    bsr r2, r0                 ; find position of last 1 bit
    store [result_bsr], r2     ; save result

    ; Test bit test operations
    mov r3, 5                  ; bit position to test
    bt r0, r3                  ; test bit 5
    mov r4, 0
    cmovs r4, 1                ; set r4 = 1 if bit was set
    store [result_bt], r4

    ; Test bit test and set
    mov r5, 0b1000             ; start with small value
    mov r6, 2                  ; bit position
    bts r5, r6                 ; set bit 2, should make r5 = 0b1100

    ; Test bit test and reset
    btr r5, r6                 ; reset bit 2, should make r5 = 0b1000

    ; Test bit test and complement
    btc r5, r6                 ; complement bit 2, should make r5 = 0b1100
    btc r5, r6                 ; complement bit 2 again, should make r5 = 0b1000

    ; Test shift operations
    mov r7, 0b11110000
    shl r7, 4                  ; shift left 4 bits

    mov r8, 0b11110000
    shr r8, 4                  ; shift right 4 bits

    ; Test arithmetic shifts
    mov r9, -16                ; negative number
    sar r9, 2                  ; arithmetic right shift (preserve sign)

    mov r10, 16
    sal r10, 2                 ; arithmetic left shift

    ; Test rotate operations
    mov r11, 0b11000011
    rol r11, 4                 ; rotate left 4 positions

    mov r12, 0b11000011
    ror r12, 4                 ; rotate right 4 positions

    ; Test rotate through carry
    mov r13, 0b10000001
    rcl r13, 1                 ; rotate left through carry
    rcr r13, 1                 ; rotate right through carry

    ; Test bit extract (if supported)
    mov r14, 0b1111000011110000
    bextr r15, r14, 0x0804     ; extract 4 bits starting at bit 8

    ; Test bitwise AND NOT
    load r16, [bit_mask]
    andn r17, r16, r0          ; r17 = ~r16 & r0

    ; Complex bit manipulation example
    mov r18, 0b10101010
    mov r19, 3                 ; bit position

    ; Check if bit is set
    bt r18, r19
    mov r20, 0
    cmovs r20, 1               ; r20 = 1 if bit 3 was set

    ; Toggle the bit
    btc r18, r19

    ; Count leading zeros by finding highest bit
    mov r21, 0b00001111
    bsr r22, r21               ; find highest set bit
    mov r23, 63
    sub r23, r22               ; calculate leading zeros (for 64-bit)

    ; Exit with combined result
    add r0, r1
    add r0, r2
    syscall exit
