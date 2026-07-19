use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No input file specified.");
        process::exit(1);
    }

    let filename = &args[1];
    if !Path::new(filename).exists() {
        eprintln!("Error: The file '{}' could not be found.", filename);
        process::exit(1);
    }

    let source_code = fs::read_to_string(filename).expect("Failed to read file");

    match compile_flux_to_asm(&source_code, "boot.asm") {
        Ok(_) => println!("Successfully compiled '{}' to 'boot.asm'!", filename),
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            process::exit(1);
        }
    }
}

fn compile_flux_to_asm(source: &str, output_path: &str) -> io::Result<()> {
    let mut file = File::create(output_path)?;

    // 1. Bootloader Setup
    writeln!(file, "[bits 16]")?;
    writeln!(file, "[org 0x7c00]\n")?;
    writeln!(file, "start:")?;
    
    // Clear segment registers to ensure stability
    writeln!(file, "    xor ax, ax")?;
    writeln!(file, "    mov ds, ax")?;
    writeln!(file, "    mov es, ax\n")?;

    let mut string_counter = 0;
    let mut string_definitions = Vec::new();

    // 2. Parse Lines for Double-Quoted Strings
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("print(\"") && trimmed.ends_with("\");") {
            if let Some(start) = trimmed.find("\"") {
                if let Some(end) = trimmed.rfind("\"") {
                    if start != end {
                        let text_content = &trimmed[start + 1..end];
                        
                        // Create a distinct memory label for this specific string
                        let label_name = format!("msg_{}", string_counter);
                        
                        // Emit assembly to load string address and call print routine
                        writeln!(file, "    mov si, {}      ; Load string address into SI register", label_name)?;
                        writeln!(file, "    call print_string    ; Run the global print loop")?;
                        
                        // Store the actual string data block to write at the end of the file
                        string_definitions.push((label_name, text_content.to_string()));
                        string_counter += 1;
                    }
                }
            }
        }
    }

    // Halt execution after running all commands
    writeln!(file, "    jmp $                ; Infinite loop to halt CPU\n")?;

    // 3. Shared Low-Level Print String Routine
    writeln!(file, "print_string:")?;
    writeln!(file, "    mov ah, 0x0e         ; BIOS teletype sub-function")?;
    writeln!(file, ".loop:")?;
    writeln!(file, "    lodsb                ; Load byte from SI into AL, increment SI")?;
    writeln!(file, "    cmp al, 0            ; Check if character is null terminator (0)")?;
    writeln!(file, "    je .done             ; If zero, jump out of loop")?;
    writeln!(file, "    int 0x10             ; Call BIOS video interrupt to print")?;
    writeln!(file, "    jmp .loop            ; Next character")?;
    writeln!(file, ".done:")?;
    writeln!(file, "    ret\n")?;

    // 4. Write Data Section containing the text constants
    for (label, text) in string_definitions {
        writeln!(file, "{} db \"{}\", 0", label, text)?;
    }

    // 5. Pad file to boot signature standard boundary
    writeln!(file, "\ntimes 510 - ($ - $$) db 0")?;
    writeln!(file, "dw 0xaa55")?;

    Ok(())
}