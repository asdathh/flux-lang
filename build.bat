@echo off
cls
echo ===================================================
echo   LAUNCHING FLUX BARE-METAL KERNEL PIPELINE        
echo ===================================================

:: 1. Run your Flux compiler to output the raw assembly text file
flux main.fx

:: 2. Assemble the text file into a bootable 512-byte binary image
nasm -f bin boot.asm -o boot.bin

:: 3. Launch the QEMU emulator container using the compiled binary
echo Booting virtual machine hardware environment...
qemu-system-x86_64 -drive format=raw,file=boot.bin

pause