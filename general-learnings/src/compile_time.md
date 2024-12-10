# What is a compiler time? 

When a compiler compiles the source code the compiler time is the time it takes to convert the code into binary code or machine code. However after the compilation the code is executable. 

## What are the key concepts of a compiler?   

1. **Lexical Analysis**: The compiler reads the source code and converts it into tokens. Tokens are the smallest units of meaning, like keywords, operators, and identifiers.

2. **Syntax Analysis**: The compiler checks the tokens against the grammatical rules of the programming language, creating a parse tree or syntax tree.

3. **Semantic Analysis**: The compiler ensures that the syntax tree adheres to the semantic rules of the language, like type checking and scope resolution.
Hello, welco
4. **Optimization**: The compiler may optimize the code to improve performance, such as reducing the number of instructions or eliminating unnecessary code.

5. **Code Generation**: The compiler translates the optimized syntax tree into machine code or intermediate code.

6. **Code Linking**: The compiler links different modules or libraries into a single executable program.


All the analysis steps can be classed as parsing. 

## What are the pros and cons? 

Performance, error detection and optimization are the benefits. Error detection for syntax errors and type errors. The cons could be longer development cycle, less flexibility as every change you will have to re-compiler the source code. 




### Examples in the real world. 

1. V8 Compiler. 
    - Googles open source javascript and webAssembly engine. Compiles javascript code into machine code. v8 uses a combination of a interpete and a Just in time. 
2. Rust Compiler
    - The Rust Compiler (rustc) is responsible for compiling Rust code into machine code. Rust is known for its focus on performance and safety. The compiler performs various checks to ensure memory safety and thread safety, leveraging Rust's ownership model and type system. It also optimizes the code for performance, producing efficient machine code.
3. Typescript
    - This compiles the typescript code into javascript
4. GCC: 
    - Standard Compliance, Error Handling,
    Preprocessor Support. C can generate code for different platforms, windows, Linux and Mac. 
5. Java Development Kit
    -  javac is the official Java compiler developed by Oracle Corporation. It turns Java into bytecode that can be executed by the Java Virtual Machine. There are various different versions of the JDK. 
