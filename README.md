# DIS - Dumb Instruction System

vscode extension - [https://github.com/xTarzx/dis-code](https://github.com/xTarzx/dis-code)


## Syntax

-   COMMENTS: `-`
    lines starting with '-' are ignored

    ```
      - this is a comment
      add 2 #2
    ```

-   NUM: `<number>`
    ```
      13
      - value 13
    ```
-   CHR: `.<character>`
    ```
      .c
      - ascii value of c
    ```
-   REG: `#<register index>`

    ```
      #0
      - register 0
    ```

-   MEM:

    -   ADR: `&<address>`

    ```
      &20
      - memory at address 20
    ```

    -   REG: `&#<register index>`

    ```
      &#1
      - memory at address of value in register 1
    ```

-   LBL: `<label>`
    labels are set in beggining of line
    lines can have just a label
    ```
      label0:
      label1: add 1 #0
      jmp label0
    ```

## Instructions

-   mov: `mov <NUM | REG | MEM | CHR> <REG | MEM>`

    ```
      mov 69 #0
      - set register 0 value to 69

      mov #1 &#0
      - set memory at address of value in register 0 to value in register 1
    ```

-   add: `add <NUM | REG | MEM | CHR> <REG | MEM>`

    ```
      add 3 #0
      - increment value in register 0 by 3

      add .a #1
      - increment value in register 0 by ascii valye of 'a' (97)
    ```

-   sub: `sub <NUM | REG | MEM | CHR> <REG | MEM>`

    ```
      sub 3 #0
      - dec value in register 0 by 3

      sub .a #1
      - dec value in register 0 by ascii valye of 'a' (97)
    ```

-   cmp: `cmp <NUM | REG | MEM | CHR> <REG | MEM>`
    sets comparison bits: `><=`

    ```
      cmp 13 #1
      - compare 13 with value in register 1

      cmp #0 &#3
      - compare value in register 0 with value at memory address of value in register 3
    ```

-   jlt: `jlt <LBL>`
    jump if less than

    ```
      jlt label0
      - jump to label0 if '<' bit is set
    ```

-   jgt: `jgt <LBL>`
    jump if greater than

    ```
      jgt label0
      - jump to label0 if '>' bit is set
    ```

-   jeq: `jeq <LBL>`
    jump if equal

    ```
      jeq label0
      - jump to label0 if '=' bit is set
    ```

-   jne: `jne <LBL>`
    jump if not equal

    ```
      jne label0
      - jump to label0 if '=' bit is not set
    ```

-   jmp: `jmp <LBL>`
    jump to label

    ```
      jmp label0
      - jump to label0
    ```

-   run: `run <LBL>`
    push current instruction address to stack and jump to label

    ```
    run label0
    ```

-   ret: `ret`
    pops address from stack and jumps to it

    ```
    run label0

    label0:
    ret
    ```

-   die: `die`
    ends program

    ```
    die
    - this wont run
    prt #0

    ```

-   out: `out <NUM | REG | MEM | CHR>`
    print char
    ```
      mov 97 #0
      out #0
      - prints 'a'
    ```
-   prt: `prt <NUM | REG | MEM | CHR>`
    print value

    ```
      mov 97 #0
      out #0
      - prints '97'
    ```

-   @ (include): `@ <filename>`

    -   includes the content at that location<br>
    -   filename must be without extension, it will append '.dis'

    ```
      @ hello
      - includes the file "hello.dis"
    ```

-   rdn: `rdn <MEM | REG>`
    read from stdin and parse as number
    sets #e

    ```
    rdn #0
    - 1 enter
    prt #0
    - outputs '1'
    ```

-   rdc: `rdc <MEM | REG>`
    read from stdin and parse first char as number
    sets #e

    ```
    rdc #0
    - 1 enter
    prt #0
    - outputs '49'
    ```

-   rln: `rln <MEM> <NUM | REG | MEM>`
    read line to address and return read count on #3<br>
    arg: max characters to read (set to zero to read until newline)

    ```
    rln &0
    - stdin: test
    - &0 &1 &2 &3
    -  t  e  s  t

    rln &10, 2
    - stdin: test
    - &10 &11 &12 &13
    -  t    e   0   0
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
