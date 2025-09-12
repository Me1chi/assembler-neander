# Neander

Neander is a very simple computer, with a minimal (really) instruction set. It was created by Dr. Raul Fernando Weber as an educational resource for INF-UFRGS students.

## Why this project?

Neander, unlike its big brothers (Ahmes, Ramses and Cesar) does not
have an assembler. Altough I'd say this is intentional, because its opcodes are not that difficult to deal with. I couldn't resist implementing this as a "side quest".

## About the assembler

(How to use it...)

### Directives
There're, for now, only two directives, these are:

| Directive | Meaning |
|:-----:|:---------------------:|
| .code | Where the code starts |
| .data | Where the data starts |

They're simple and are used to organize the memory into
two pieces. Mem(127..0) = Code, Mem(255..128) = Data.

### Relevant symbols

| Symbol | Meaning |
|:-----:|:---------------------:|
| label: | A label is used to "label" an address |
| label_with_no_colon | An already declared label as an operand |
| $num | The '$' means immediate access mode (read the note below) |

 - Please note that Neander itself is not compatible with immediate
 addressing. Knowing sometimes it is needed and you have no choice but setting, for example, a thing like two: 2 (a label called two, which's address contains the value literal value). 
 - The situation described is exactly how the assembler will proceed when a $x is found. So, you must keep in mind that every $x means one less byte from the few 128 available for data.

### The instruction set

| Instruction | Operation |
|:-----:|:---------------------:|
| NOP  | No operation |
| STA addr | Stores the value of the accumulator at addr |
| LDA addr | Loads the value at addr into the accumulator |
| ADD addr | Adds the value at addr to the accumulator |
| OR addr | Performs a logical OR from addr to the accumulator |
| AND addr | Performs a logical AND from addr to the accumulator |
| NOT | Performs a logical NOT on the accumulator |
| JMP addr | Unconditional jump to addr |
| JZ addr | Jump to addr if Zflag = 1 |
| JN addr | Jump to addr if Nflag = 1 |
| HLT | Halts the program |

### More info / Downloads
Further and technical information, as well as the Neander simulator download, can be found at this [link](https://www.inf.ufrgs.br/arq/wiki/doku.php?id=neander)


