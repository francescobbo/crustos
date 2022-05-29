ISO := crustos.iso

all: $(ISO)

$(ISO):
	cargo build
	mv target/x86_64-crustos/debug/crustos iso/boot
	grub-mkrescue -o $(ISO) iso

qemu: $(ISO)
	qemu-system-x86_64 -boot d -cdrom $(ISO)