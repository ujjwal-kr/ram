# The Ram programming language
A stack based programming language created to experiment my crappy lang-dev only capable of making some mathematical formulation. Written in the rust programming language.

## How to run
1. Download the [release binary](https://github.com/ujjwal-kr/ram/releases/download/v1.2/ram).
2. Make it executable using `chmod +x ./ram`
3. Make a .ram file with the following contents at the same directory as the binary:

```
printc >> Coin Flip
rand >> 0,1
round
ram 1
cmp
je 1
pop
ram 0
cmp
je 2

printc >> Heads

printc >> Tails
```

4. run `./ram` and enter the file path eg. `example.ram` at the prompt.
5. Die

## Documentation
Official documentation of the RAM programming language. Please open an issue if you want some features to be chaged or added.

### print
- `print` prints the last number on the screen present in the stack.
- `print lx/rv` prints the value of `lx/rv` variables on the screen.

### printc >> characters
Prints the characters specified on the screen

### ram
- `ram <number>` adds the number to the stack.
- `ram lx/rv <number>` assigns the number to the variable
- `ram lx/rv` adds the variable value to the stack for further calculations
- `ram lx/rv prev` assigns the last stack value to the specified variable.
     
### pop
Removes the last value from the stack

### popall
Empties the entire stack. Useful for `avg` implementation.

### add
- adds last two numbers in the stack and pushes the result to the stack
- `add lx rv` adds the two variables and pushes the result to the stack

### sub 
subtracts the last number from the number added prior to it and pushes to stack

### mul
- `mul` multiplies the last two numbers in the stack and adds the result to the stack
- `mul rv lx` multiplies lx and rv and pushes the result to the stack.

### div
the number added prior to the last number / last number and pushes to stack.

### sqr
- `sqr` squares the last number in the stack and adds the result to the stack.
- `sqr lx/rv` squares `lx/rv` and assigns the result to the variable squared.

### sqrt
- `sqrt` squares the last number in the stack and adds the result to the stack.
- `sqrt lx/rv` squares `lx/rv` and assigns the result to the variable squared.

### rand >> num1,num2
`rand >> num1,num2` generates a random decimal between num1 and num2 (including both) and pushes it to stack. May `round` be used after this statement to round it to an integer.

### round
- `round` rounds the last number present in the stack to decimals and pushes it to the stack.
- `round lx/rv` rounds the specified variable and assigns the result to the var itself.

### avg
Takes out the average of all the numbers present in the stack. `popall` should be used somewhere in the code to remove the vars which are redundant.

### `cmp` and Jump statements
The program is devided into blocks separated by empty lines. And each block has an index. For example-
```
ram 100
ram 300
cmp
jne 1
cmp
je 2

printc >> Not Equal

printc >> Equal
```

This code consists of three codeblocks, indexed 0,1,2 respectively and can be accessed by the jump statements.

- `jmp <index>` - jumps to the codeblock at the index position.
- `cmp` pushes -1, 0, 1 to the stack if the last two numbers are smaller, equal or greater than each other. (Important before jne and je commands).
- `jne <index>` - jumps to a block by its index position if the previous cmp statement is not 0 (equal)

- `je <index>` - jumps to a block by its index position if the previous cmp statement is 0 (equal)

- `jgr <index>` - jumps to a block by its index position if the prev cmp statement is 1 (greater)

- `jsm <index>` - jumps to a block by its index position if the prev cmp statement is -1 (smaller)


