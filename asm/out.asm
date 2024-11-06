section .text
    global _start
_start:
    mov rax, 60
    movzx rdi, byte [value]
    syscall

section .data
value db 5

section .bdd
