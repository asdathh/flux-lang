[bits 16]
[org 0x7c00]

start:
    xor ax, ax
    mov ds, ax
    mov es, ax

    mov si, msg_0      ; Load string address into SI register
    call print_string    ; Run the global print loop
    mov si, msg_1      ; Load string address into SI register
    call print_string    ; Run the global print loop
    jmp $                ; Infinite loop to halt CPU

print_string:
    mov ah, 0x0e         ; BIOS teletype sub-function
.loop:
    lodsb                ; Load byte from SI into AL, increment SI
    cmp al, 0            ; Check if character is null terminator (0)
    je .done             ; If zero, jump out of loop
    int 0x10             ; Call BIOS video interrupt to print
    jmp .loop            ; Next character
.done:
    ret

msg_0 db "Aero OS is loading...", 0
msg_1 db " Kernel active.", 0

times 510 - ($ - $$) db 0
dw 0xaa55
