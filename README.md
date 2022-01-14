# The Ram programming language
A stack based programming language created to experiment my crappy lang-dev only capable of making some mathematical formulation. Written in the rust programming language.

## How to run
1. Download the [release binary](https://github.com/ujjwal-kr/ram/releases/download/v1.1/ram).
2. Make it executable using `chmod +x ./ram`
3. Make a .ram file with the following contents at the same directory as the binary:

```
ram 500
ram 200
add
print
```

4. run `./ram` and enter the file name eg. `example.ram` at the prompt.
5. Die

## More documentation

```
ram <number>  - pushes a number on the stack
pop - removes the last number added to the stack
popall - removes all the numbers from the stack
add - adds last two numbers pushed on the stack and pushes to stack
sub - subtracts the last number from the number added prior to it and pushes to stack
mul - subtracts the last number from the number added prior to it and pushes to stack
div - the number added prior to the last number / last number and pushes to stack
sqr - squares the last number and pushes to stack
squrt - square roots the last number and pushes to stack
round - rounds the last number pushed in the stack to decimals and pushes it to stack
avg - takes out average of all the numbers in the stack (remember to run popall before it) and pushes to stack
cmp - pushes -1, 0, 1 to the stack based two numbers are smaller, equal or greater than each other. [Important before jne and je commands]
jne <index> - jumps to a block by its index position if the previous cmp statement is not 0 [equal]
je <index> - jumps to a block by its index position if the previous cmp statement is 0 [equal]
jgr <index> - jumps to a block by its index position if the prev cmp statement is 1 [greater]
jsm <index> - jumps to a block by its index position if the prev cmp statement is -1 [smaller]
```
