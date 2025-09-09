section .data
byte_val db 0xFF        ; -1 as signed byte, 255 as unsigned
word_val dw 0xFFFF      ; -1 as signed word, 65535 as unsigned
dword_val dd 0xFFFFFFFF ; -1 as signed dword, 4294967295 as unsigned
qword_val dq 0x123456789ABCDEF0

; Test values for conversion
test_byte db 0x80       ; -128 as signed, 128 as unsigned
test_word dw 0x8000     ; -32768 as signed, 32768 as unsigned
test_dword dd 0x80000000 ; negative value for testing

; Results storage
result_cbw dq 0
result_cwd dq 0
result_cdq dq 0
result_cqo dq 0
result_cwde dq 0
result_cdqe dq 0

section .text
global _start

_start:
    ; Test Convert Byte to Word (CBW)
    ; Sign-extend AL to AX (8-bit to 16-bit)
    load r0, [test_byte]    ; Load byte value (0x80 = -128)
    cbw r1                  ; Convert byte in r0 to word in r1
    store [result_cbw], r1  ; Should be 0xFF80 (-128 sign-extended)

    ; Test Convert Word to Doubleword (CWD)
    ; Sign-extend AX to DX:AX (16-bit to 32-bit)
    load r0, [test_word]    ; Load word value (0x8000 = -32768)
    cwd r2                  ; Convert word in r0 to dword in r2
    store [result_cwd], r2  ; Should be 0xFFFF8000 (-32768 sign-extended)

    ; Test Convert Doubleword to Quadword (CDQ)
    ; Sign-extend EAX to EDX:EAX (32-bit to 64-bit)
    load r0, [test_dword]   ; Load dword value (0x80000000 = negative)
    cdq r3                  ; Convert dword in r0 to qword in r3
    store [result_cdq], r3  ; Should be sign-extended negative value

    ; Test Convert Quadword to Octword (CQO)
    ; Sign-extend RAX to RDX:RAX (64-bit to 128-bit, store high part)
    load r0, [qword_val]    ; Load qword value
    cqo r4                  ; Convert qword, r4 gets high part
    store [result_cqo], r4  ; High 64 bits of sign extension

    ; Test Convert Word to Doubleword Extended (CWDE)
    ; Zero-extend AX to EAX (16-bit to 32-bit unsigned)
    load r0, [word_val]     ; Load word value
    cwde r5                 ; Convert word to dword (zero-extend)
    store [result_cwde], r5 ; Should be zero-extended

    ; Test Convert Doubleword to Quadword Extended (CDQE)
    ; Sign-extend EAX to RAX (32-bit to 64-bit)
    load r0, [dword_val]    ; Load dword value
    cdqe r6                 ; Convert dword to qword (sign-extend)
    store [result_cdqe], r6 ; Should be sign-extended

    ; Demonstrate practical usage: sign extension for arithmetic
    ; Example: multiply signed byte by signed word
    load r7, [test_byte]    ; Load signed byte (-128)
    cbw r7                  ; Convert to word (-128 -> -128 in 16-bit)

    load r8, [test_word]    ; Load signed word (-32768)

    ; Now we can multiply the sign-extended values
    imul r7, r8             ; Signed multiply

    ; Another example: preparing for division
    load r9, [test_dword]   ; Load 32-bit dividend
    cdq r10                 ; Sign-extend to 64-bit for division
    ; r10 now contains the high part needed for 64-bit division

    ; Test with positive values to show zero extension
    mov r11, 0x7F           ; Positive byte
    cbw r11                 ; Should remain 0x007F

    mov r12, 0x7FFF         ; Positive word
    cwde r12                ; Should become 0x00007FFF

    ; Test boundary cases
    mov r13, 0              ; Zero
    cbw r13                 ; Should remain zero
    cwd r13                 ; Should remain zero

    mov r14, 1              ; Small positive
    cdq r14                 ; Should remain small positive with zero high bits

    ; Chained conversions example
    mov r15, 0x80           ; Start with byte
    cbw r15                 ; Byte to word
    cwde r15                ; Word to dword
    cdqe r15                ; Dword to qword

    ; Final result combines all conversions
    add r0, r1
    add r0, r2
    add r0, r3
    add r0, r15

    ; Exit with combined result
    and r0, 0xFF            ; Keep only low byte for exit code
    syscall exit
