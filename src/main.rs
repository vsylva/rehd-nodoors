struct Game {
    process: &'static str,
    signature: &'static str,
    patches: &'static [Patch],
}

struct Patch {
    address: usize,
    opcodes: &'static [u8],
}

const RE0: Game = Game {
    process: "re0hd.exe",
    signature: "F3 0F 10 40 38 F3 0F 59 05 14 A4 CB 00 F3",
    patches: &[
        Patch {
            address: 0x552A13,
            opcodes: &[
                0xC7, 0x47, 0x2C, 0x00, 0x00, 0x80, 0xBF, 0xF3, 0x0F, 0x10, 0x47, 0x2C, 0xEB, 0x1C,
                0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
                0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
            ],
        },
        Patch {
            address: 0x553130,
            opcodes: &[0xC3, 0x90, 0x90],
        },
        Patch {
            address: 0x552630,
            opcodes: &[0x90, 0x90, 0x90, 0x90, 0x90, 0x90],
        },
    ],
};

const RE: Game = Game {
    process: "bhd.exe",
    signature: "8B 46 48 85 C0",
    patches: &[
        Patch {
            address: 0x41CD83,
            opcodes: &[0xE9, 0x9F, 0x00, 0x00, 0x00],
        },
        Patch {
            address: 0x41CF35,
            opcodes: &[0xE9, 0x7E, 0x00, 0x00, 0x00],
        },
        Patch {
            address: 0x41D10F,
            opcodes: &[
                0x5F, 0xC7, 0x86, 0x84, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x5E, 0x5D, 0x5B,
                0xC2, 0x10, 0x00,
            ],
        },
        Patch {
            address: 0x611A1A,
            opcodes: &[0xFA],
        },
    ],
};

fn main() {
    if let Err(err) = patch(RE0) {
        println!("{err}");
    } else {
        println!("Patched RE0");
    }

    if let Err(err) = patch(RE) {
        println!("{err}");
    } else {
        println!("Patched RE");
    }

    press_enter_to_continue()
}

fn press_enter_to_continue() {
    print!("\nPress ENTER to continue...");
    ::std::io::Write::flush(&mut ::std::io::stdout()).unwrap_or_default();

    ::std::io::stdin()
        .read_line(&mut String::new())
        .unwrap_or_default();
}

fn patch(game: Game) -> Result<(), Box<dyn ::std::error::Error>> {
    let Some(process) = libmem::find_process(game.process) else {
        return Err(format!("Find {} process err", game.process).into());
    };

    let Some(module) = libmem::find_module_ex(&process, &process.name) else {
        return Err(format!("Find {} module err", process.name).into());
    };

    if libmem::sig_scan_ex(&process, game.signature, module.base, module.size).is_none() {
        return Err(format!("Scan {} signature err", process).into());
    }

    for patch in game.patches {
        if libmem::write_memory_ex(&process, patch.address, patch.opcodes).is_none() {
            return Err(format!("Write {} patch err", process).into());
        }
    }

    Ok(())
}
