# DIS - Dumb Instruction System

## Syntax

- COMMENTS: `-`
  lines starting with '-' are ignored

  ```
  - this is a comment
  add 2 #2
  ```

- NUM: `<number>`
  ```
  13
  value 13
  ```
- CHR: `.<character>`
  ```
  .c
  ascii value of c
  ```
- REG: `#<register index>`

  ```
  #0
  register 0
  ```

- MEM:

  - ADR: `&<address>`

  ```
    &20
    memory at address 20
  ```

  - REG: `&#<register index>`

  ```
  &#1
  memory at address of value in register 1
  ```

- LBL: `<label>`
  labels are set in beggining of line
  lines can have just a label
  ```
  label0:
  label1: add 1 #0
  jmp label0
  ```

## Instructions

- MOV: `mov <NUM | REG | MEM> <REG | MEM>`

  ```
    mov 69 #0
    set register 0 value to 69

    mov #1 &#0
    set memory at address of value in register 0 to value in register 1
  ```

- ADD: `add <NUM | REG | MEM | CHR> <REG | MEM>`

  ```
    add 3 #0
    increment value in register 0 by 3

    add .a #1
    increment value in register 0 by ascii valye of 'a' (97)
  ```

- CMP: `cmp <NUM | REG | MEM | CHR> <REG | MEM>`
  sets comparison bits: `><=`

  ```
    cmp 13 #1
    compare 13 with value in register 1

    cmp #0 &#3
    compare value in register 0 with value at memory address of value in register 3
  ```

- JLT: `jlt <LBL>`
  jump if less than

  ```
    jlt label0
    jump to label0 if '<' bit is set
  ```

- JGT: `jgt <LBL>`
  jump if greater than

  ```
    jgt label0
    jump to label0 if '>' bit is set
  ```

- JEQ: `jeq <LBL>`
  jump if equal

  ```
    jeq label0
    jump to label0 if '=' bit is set
  ```

- JNE: `jne <LBL>`
  jump if not equal

  ```
    jne label0
    jump to label0 if '=' bit is not set
  ```

- JMP: `jmp <LBL>`
  jump to label

  ```
    jmp label0
    jump to label0
  ```

- RUN: `run <LBL>`
  push current instruction address to stack and jump to label

  ```
  run label0
  ```

- RET: `ret`
  pops address from stack and jumps to it

  ```
  run label0

  label0:
  ret
  ```

- DIE: `die`
  ends program

- OUT: `out <REG | MEM>`
  print char
  ```
    mov 97 #0
    out #0
    prints 'a'
  ```

## Example

hello.dis

```
mov .H  &0
mov .e  &1
mov .l  &2
mov .l  &3
mov .o  &4
mov 10  &5
mov 0 #0
print: out &#0
add 1 #0
cmp 0 &#0
jne print
```
