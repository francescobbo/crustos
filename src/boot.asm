[section .multiboot]
[global header_start]
align 8
header_start:
    dd 0xe85250d6  
    dd 0           
    dd header_end - header_start 

    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    dw 0
    dw 0
    dd 8
header_end:

%define LOWER(address) (address - 0xffffffff80000000)

[section .text]
[global _start]
[bits 32]
_start:
    cli
    cld

    ; This mov is here only to guarantee that the .multiboot section makes it
    ; into the executable. The linker might otherwise think it's useless.
    mov eax, LOWER(header_start)

    ; Save the multiboot information pointer
    mov ecx, LOWER(multiboot_info)
    mov [ecx], ebx

    ; Set up a page directory using 2MiB pages
    ; Add more pages until we have fully mapped the kernel image

    mov eax, 0
    mov ebx, LOWER(pml2)

[extern low_kernel_image_end]
.directory_loop:
    ; ecx = current address
    mov ecx, eax
    ; 0x83 = Present and "Large" (2MiB)
    or ecx, 0x83

    mov [ebx], ecx

    add eax, 0x200000
    add ebx, 8

    ; Break the loop if we mapped up to the end of the kernel
    cmp eax, low_kernel_image_end
    jg .done

    ; Repeat
    jmp .directory_loop

.done:
    mov edi, LOWER(kernel_pml4)
    mov cr3, edi

    ; Enable the Physical Address Extension
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; Prepare for Long Mode by setting bit #8 of IA32_EFER
    mov ecx, 0xc0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; Enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ; Set up a 64 bit GDT
    mov eax, LOWER(gdtr)
    lgdt [eax]

    ; Enter long mode by far jumping to the 64 bit code segment
    jmp 0x08:LOWER(longmode)

[bits 64]
longmode:
    ; We're now in long mode, but in the low, identity-mapped memory portion.
    ; We can now use 64 bit addresses...
    ; Immediately jump to high memory (> 0xffffffff80000000)
    mov rax, higher
    jmp rax

[extern main]
higher:
    ; Set up 64 bit data segments
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax

    ; Set up the boot kernel stack
    mov rsp, stack

    ; End the backtrace chain by zeroing the frame pointer
    xor rbp, rbp

    ; The GDTR base address is paging-aware, so before disabling the identity
    ; map, reload it with a higher memory address.
    mov rax, gdtr64
    lgdt [rax]

    ; Cleanup low memory identity map
    mov rdi, kernel_pml4
    mov qword [rdi], 0

    mov rdi, kernel_pml3
    mov qword [rdi], 0

    ; Flush the paging translations for low memory
    invlpg [0]

    mov rcx, multiboot_info
    mov rdi, [rcx]
    add rdi, 0xffffffff80000000

    ; Stack should be 16-byte aligned at this point
    call main

    ; Should never be reached
    cli
    hlt

[section .rodata]
gdt:
    dq 0
    dq 0x00af9a000000ffff
    dq 0x008f92000000ffff

gdtr:
    dw 23
    dd LOWER(gdt)

gdtr64:
    dw 23
    dq gdt

[section .data]

; Paging structures must be page-aligned
align 0x1000

; The page directory (level 2) that holds kernel code and data at boot.
pml2:
    times 512 dq 0

; 512GiB virtual memory block for kernel exclusive use
; Starting at 0xffffff80_00000000
[global kernel_pml3]
kernel_pml3:
    ; Temporary identity mapping of lower memory
    dq LOWER(pml2) + 3

    times 509 dq 0

    ; 1 GiB virtual address space starting at 0xffffffff_80000000
    dq LOWER(pml2) + 3

    ; 1 GiB starting at 0xffffffff_c0000000. Unused at boot
    dq 0

; Root paging structure used for Kernel threads
[global kernel_pml4]
kernel_pml4:
    ; Temporary identity mapping of lower memory
    dq LOWER(kernel_pml3) + 3

    times 509 dq 0

    ; PML4 self-referencing entry. This is later used to manage virtual memory
    dq LOWER(kernel_pml4) + 3

    ; Actual virtual memory mapping for the kernel
    dq LOWER(kernel_pml3) + 3

[section .bss]
align 16
resb 16 * 1024
stack:

multiboot_info: resb 8
