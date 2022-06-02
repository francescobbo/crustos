#!/bin/bash

rm -f iso/boot/crustos
cp $1 iso/boot/crustos
grub-mkrescue -o crustos.iso iso
qemu-system-x86_64 -boot d -cdrom crustos.iso \
    -serial stdio \
    --no-reboot \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04
if [ $? -eq "33" ]; then
    exit 0
else
    exit 1
fi