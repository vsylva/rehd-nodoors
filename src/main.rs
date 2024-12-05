struct Game {
    name: &'static str,
    pattern: &'static str,
    patches: &'static [Patch],
}

struct Patch {
    addr: usize,
    data: &'static [u8],
}

const RE0: Game = Game {
    name: "re0hd.exe",
    pattern: "F3 0F 10 40 38 F3 0F 59 05 64 A4 CB 00 F3",
    patches: &[
        Patch {
            addr: 0x552B93,
            data: &[
                0xC7, 0x47, 0x2C, 0x00, 0x00, 0x80, 0xBF, 0xF3, 0x0F, 0x10, 0x47, 0x2C, 0xEB, 0x1C,
                0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
                0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
            ],
        },
        Patch {
            addr: 0x5532B0,
            data: &[0xC3, 0x90, 0x90],
        },
        Patch {
            addr: 0x5527B0,
            data: &[0x90, 0x90, 0x90, 0x90, 0x90, 0x90],
        },
    ],
};

const RE: Game = Game {
    name: "bhd.exe",
    pattern: "8B 46 48 85 C0",
    patches: &[
        Patch {
            addr: 0x41CD83,
            data: &[0xE9, 0x9F, 0x00, 0x00, 0x00],
        },
        Patch {
            addr: 0x41CF35,
            data: &[0xE9, 0x7E, 0x00, 0x00, 0x00],
        },
        Patch {
            addr: 0x41D10F,
            data: &[
                0x5F, 0xC7, 0x86, 0x84, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x5E, 0x5D, 0x5B,
                0xC2, 0x10, 0x00,
            ],
        },
        Patch {
            addr: 0x611A19 + 1,
            data: &[0xFA],
        },
    ],
};

fn main() {
    if let Err(err) = patch(RE0) {
        println!("{err}");
    } else {
        println!("successful patch RE0");
    }

    if let Err(err) = patch(RE) {
        println!("{err}");
    } else {
        println!("successful patch RE");
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
    let Some(process) = libmem::find_process(game.name) else {
        return Err(format!("failed to find the process {}", game.name).into());
    };

    let Some(module) = libmem::find_module_ex(&process, &process.name) else {
        return Err(format!("failed to find the module {}", process.name).into());
    };

    if libmem::sig_scan_ex(
        &process,
        // version pattern
        game.pattern,
        module.base,
        module.size,
    )
    .is_none()
    {
        return Err("failed to scan the sig".into());
    }

    for patch in game.patches {
        if libmem::write_memory_ex(&process, patch.addr, patch.data).is_none() {
            return Err("failed to write the patch codes".into());
        }
    }

    Ok(())
}
