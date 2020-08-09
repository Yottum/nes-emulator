## Instructions
- [x] ADC
- [x] AND
- [x] ASL
- [x] BCC
- [x] BCS
- [x] BEQ
- [x] BIT
- [x] BMI
- [x] BNE
- [x] BPL
- [x] BRK
- [x] BVC
- [x] BVS
- [x] CLC
- [x] CLD
- [x] CLI
- [x] CLV
- [x] CMP
- [x] CPX
- [x] CPY
- [x] DEC
- [x] DEX
- [x] DEY
- [ ] EOR
- [ ] INC
- [ ] INX
- [ ] INY
- [ ] JMP
- [ ] JSR
- [x] LDA
- [x] LDX
- [x] LDY
- [ ] LSR
- [x] NOP
- [ ] ORA
- [x] PHA
- [x] PHP
- [x] PLA
- [ ] PLP
- [ ] ROL
- [ ] ROR
- [ ] RTI
- [ ] RTS
- [ ] SBC
- [x] SEC
- [x] SED
- [x] SEI
- [x] STA
- [x] STX
- [x] STY
- [x] TAX
- [x] TAY
- [x] TSX
- [x] TXA
- [x] TXS
- [x] TYA

## Miscellaneous
- [ ] Fix IndirectY, currently a copy of IndirectX, but should have separate indirection logic
- [ ] Check if status flags modified during instructions are only set if relevant, or _always_ overridden (latter is currently the case)
- [ ] Check overflow and wrapping rules for each instruction
