[![Main Status](https://img.shields.io/github/workflow/status/ericwoude/gameboy-disassembler/main?style=social)](https://github.com/ericwoude/gameboy-disassembler/actions/workflows/main.yml)
[![License](https://img.shields.io/github/license/ericwoude/gameboy-disassembler?style=social)](https://github.com/ericwoude/gameboy-disassembler/blob/main/LICENSE)

# Game Boy disassembler
> Disassembler for the Nintendo Gameboy DMG-01

## Usage
### Help
```
$ ./gameboy-disassembler --help
Usage: gameboy-disassembler [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>      Relative path pointing to GameBoy ROM
  -e, --entry <ENTRY>    Entry address for disassembly [default: 336]
  -a, --amount <AMOUNT>  Amount of instructions to decode from starting point [default: 8]
  -h, --help             Print help information
```

### Running the disassembler
```
$ ./gameboy-disassembler --file ./snake.gb --entry 0x150
0x150 NOP
0x151 DI
0x152 LD SP, 0xFFFE
0x155 LD B, 0x80
0x157 LD C, 0x0
0x159 LDH A, (0x44)
0x15B CP 0x90
0x15D JR NZ, 0xFA
```


Credits for the test rom go to Yvar de Goffau [snake.gb](https://forums.nesdev.org/viewtopic.php?f=20&t=16787)