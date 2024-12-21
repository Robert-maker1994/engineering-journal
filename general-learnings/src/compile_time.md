# What is Compile Time?

When we talk about compile time, we're referring to the duration it takes for a compiler to convert source code into machine code, or binary code. Once the compilation is complete, the code is ready to be executed.

## What Does a Compile Do?

A compiler goes through several steps before producing machine code. We can split the phases of a compiler into three main phases: the Analysis phase, the Optimization phase, and the Code Generation phase. Keep in mind that errors can occur at any phase.

### Analysis Phase

In the Analysis phase, we focus on Lexical, Syntax, and Semantic analysis. This phase involves analyzing the source code to understand its structure and meaning.

- **Lexical Analysis**: The compiler converts the source code into tokens, which represent meaningful elements such as keywords, identifiers, operators, or punctuation. Errors such as illegal characters or malformed tokens can occur here.
- **Syntax Analysis**: Also known as parsing, this step uses the tokens to construct a syntax tree, representing the grammatical structure of the code. Syntax errors are detected in this phase.
- **Semantic Analysis**: This step checks for type mismatches, undeclared variables, and other semantic errors. It ensures that the code adheres to the language's rules.

After the Analysis phase, the code is typically represented in an Abstract Syntax Tree (AST), which moves us to the next phase.

### Optimization Phase

The Optimization phase improves the intermediate code for performance by applying techniques like constant folding, dead code elimination, and loop optimization. The goal is to make the code run faster and use fewer resources. Errors in this phase can be due to memory leaks or mis-optimization.

### Code Generation Phase

The final stage of the compiler is code generation, where the optimized intermediate code is translated into machine code or executable code.

## Why Do We Need Compilers?

Compilers are essential for several reasons:
- **Efficient Execution**: Compiled code runs much faster due to the optimizations performed during the compilation process.
- **Portability**: Compilers help us move code across different platforms and operating systems.
- **High-Level Language Translation**: Compilers translate high-level languages into machine-readable code, making it easier for developers to write and deploy code.

### Examples of Compilers in the Real World

- **GNU Compiler Collection (GCC)**: Supports multiple languages and offers various optimization levels.
- **V8 Engine**: Used in Chromium browsers and Node.js, also allows optimization levels.
- **TypeScript Compiler**: Provides type safety, improved code maintainability, and compatibility with existing JavaScript code.
- **Rust Compiler (rustc)**: Ensures memory and thread safety, optimizing code for performance.

**Conclusion**

Compilers are a crucial part of the development process, handling parsing, optimization, and code translation to machine code.

Let's move on to runtime.

# What is Runtime?

There are two concepts of runtime that we will discuss.

The first concept of runtime refers to the period when a program is actively executing on a computer or device. During this execution phase, the CPU carries out the instructions embedded within the program’s machine code. This is when the program transforms from mere code into a functional application, performing tasks like displaying a webpage, playing a video, or processing data.

The second concept of runtime relates to the environment in which a program operates. This includes the operating system and linked code libraries that provide additional functionality and resources to the program. Without a runtime environment, a program would struggle to function effectively.

## What Happens During Runtime?

Imagine you’ve just compiled your program. What you get is a bundle of machine code, typically in the form of an executable file. This is where our journey into the runtime environment begins.

- **Memory Management**: The runtime environment handles memory allocation and deallocation, often managed by garbage collection.
- **Event Handling**: Manages user inputs like mouse clicks and keyboard presses, ensuring real-time responses.
- **Libraries and Frameworks**: Provide additional functionality, making it easier to handle tasks like network requests or managing graphics.

<!-- 
TODO 
## Real-time runtimes. 

TODO
## The difference between runtimes -->

## In Summary - What is the Difference Between Runtime and Compile Time?

I hope now you have a better understanding of compile time and runtime, and what happens under the hood.
