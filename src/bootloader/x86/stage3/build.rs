use std::{fs::File, io::Write, process::Command};

fn main() {
    // Generate Interrupt Service Routines
    let isrs_with_err = [8, 10, 11, 12, 13, 14, 17, 21, 29, 30];
    let mut file = File::create("asm/isr.asm").unwrap();
    file.write_all(b"; File generated automatically by build.rs\n")
        .unwrap();

    for i in 0..256 {
        if isrs_with_err.contains(&i) {
            writeln!(&mut file, "ISR_ERROR {}", i).unwrap();
        } else {
            writeln!(&mut file, "ISR_NO_ERROR {}", i).unwrap();
        }
    }

    let mut file = File::create("src/interrupts/isr.rs").unwrap();
    writeln!(
        &mut file,
        "// File generated automatically by build.rs, do not modify.\nuse crate::interrupts::{{set_idt_gate, IDTFlagNumeric}};\n\nextern \"cdecl\" {{"
    )
    .unwrap();
    for i in 0..255 {
        writeln!(&mut file, "    pub fn isr{} ();", i).unwrap();
    }
    writeln!(
        &mut file,
        "}}\n\npub fn initialize_interrupt_serive_routines() {{"
    )
    .unwrap();
    for i in 0..255 {
        writeln!(&mut file, "    set_idt_gate(\n        {0},\n        isr{0} as *const (),\n        0x08,\n        IDTFlagNumeric::Ring0 | IDTFlagNumeric::GateInterrupt32Bit,\n    );", i).unwrap();
    }
    writeln!(&mut file, "}}").unwrap();

    // Assemble the assembly file
    Command::new("nasm")
        .args(["-f", "elf32", "-o", "main.o", "main.asm"])
        .status()
        .expect("Failed to assemble main.asm");

    // Link the object file with Rust's output
    println!("cargo:rustc-link-arg=main.o");
    println!("cargo:rerun-if-changed=main.asm");
}
