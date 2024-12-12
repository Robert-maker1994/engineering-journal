Hey everyone! Welcome to our very first episode of "Bitesize developer." where we break down big ideas into small bytes. Today, we're diving into the world of compilers and runtimes. By the end of this episode, you'll have a clear understanding of what happens behind the scenes when your code transforms into a working program.

# What is a compiler time? 

When we talk about compiler time, we're referring to the duration it takes for a compiler to convert source code into machine code, or binary code. Once the compilation is complete, the code is ready to be executed.

## What does a compiler do? 

A compiler makes several steps before making machine code. We can split the phases of a compiler into three phases the Analysis phase, the code  optimization phase and the End phase that handles code generation and code linking. Keep in mind that all of the phases can throw an error. 

**Analysis Phase**

In the Analysis phase we care about Lexical, Syntax and semantic analysis. Analyzing the source code to understand its structure and meaning. 

The Lexical Analysis the compiler converts the source code into tokens to construct a syntax tree, which represents the grammatical structure of the code. The tokens represent a meaningful element such as a keyword, identifier, operator or punctuation. _Errors_ such as illegal characters, malformed tokens can happen here. Ensuring that the source code adheres to the lexical rules of the programming language. The second phase is Syntax Analysis (other wise know as parsing) which uses the tokens to construct a syntax tree, which represents the grammatical structure of the code. If you have guessed this is where you would pick up the Syntax errors. The final part of the the Analysis phase is semantic, which you may recognize by some of the errors which it throws here. Type mismatch and undeclared variable. The functions of semantic analysis is type, labeling and flow control checking. 

After the Analysis phase, the code is typically represented in an Abstract Syntax Tree (AST). Which moves us on to the next phase. 

**Code Generation**

Improves the intermediate code for performance, applying techniques like constant folding, dead code elimination, and loop optimization. The goal is to make the code run faster and use fewer resources. The errors that can happen here are due to memory leaks or mis-optimization. The following step is generating the code to machine code or executable code. 

**Optimization Phase**

The final stage of the compiler is optimization. This optimization is to improve the performance, such as loop unrolling, function inlining and constant folding, which is where the compiler identifies constant expressions and evaluates them at compile-time, replacing the original expression with its computed value. 


## Why we need compilers? 

We need compilers for a lot of different reasons, efficient execution of code, which run much faster achieved through the analysis phase. Portability helping us move code  across different platforms and operating systems. Translate high-level languages into machine-readable code. This make it better that a vast majority of people can write code and deploy it, being able to write code in high-level languages. 


### Examples in the real world and different types of compilers . 

If you have ever compiled languages including C, C++ you might have compiled with the GNU Compiler Collection (GCC). The cool features of the GCC is that of multi-language support and its optimization levels where you can choose if you can have no optimization and High-performance optimization. 

This is quite common, in the v8 engine that runs Chromium browsers, you can also choose the optimization levels. The v8 engine also is used in nodejs. 

Since we're talking about nodejs lets take a look at the Typescript compiler, which is a statically typed superset of javascript. Typescript compiler gives you type safety, improved code maintainability, and compatibility with existing JavaScript code. The compiler would be helpful to catch errors before they arise.  

**Rust Compiler**
    - Now the Rust Compiler (rustc) is responsible for compiling Rust code into machine code. Rust is known for its focus on performance and safety. The compiler performs various checks to ensure memory safety and thread safety, implementing and checking against Rust's ownership model and type system. It also optimizes the code for performance, producing efficient machine code.


**Conclusion**

So compilers are a crucial part of the development process. Handling a parsing, optimization and code Translation to machine code.

Lets move on to a run time. 

# **What is runtime**

There are two concepts of runtime that we will discuss. 

The first concept of runtime refers to the period when a program is actively executing on a computer or device. During this execution phase, the computer’s central processing unit (CPU) carries out the instructions embedded within the program’s machine code. This is the critical moment when the program transforms from mere code into a functional application, performing the tasks it was designed to accomplish, such as displaying a webpage, playing a video, or processing data. Essentially, this is when the program comes to life and does its job. 

The second concept of runtime relates to the environment in which a program operates. This includes the operating system, which acts as a bridge between the hardware and software, and the linked code libraries that provide additional functionality and resources to the program. Without a runtime environment, a program would struggle to function effectively. Even if the program has been correctly written and compiled, it won't run without the necessary runtime environment. This environment is crucial for executing the program and enabling user interaction, ultimately making the code functional and allowing it to fulfill its intended purpose. 

## What happens during runtime? 

So, let’s kick things off. Imagine you’ve just compiled your program. What you get is a bundle of machine code, typically in the form of an executable file with a .exe extension. This is where our journey into the runtime environment begins—whether it's the start of your latest game or the activation of a server.

The first big task for the runtime environment is to handle memory management. This means figuring out how much memory is needed for variables, data structures, and objects, and then allocating and deallocating this memory as necessary. This is often managed by something called garbage collection, which keeps things running smoothly by cleaning up memory that’s no longer in use.

But that's not all. Event handling is another crucial part of runtime. It manages everything from user inputs like mouse clicks and keyboard presses to system signals. This ensures that your program responds to interactions in real-time and let’s not forget about libraries and frameworks. These are additional pieces of code that provide extra functionality, making it easier to handle tasks like network requests or managing graphics. They extend the capabilities of your program without having to reinvent the wheel.


<!-- 
TODO 
## Real-time runtimes. 

TODO
## The difference between runtimes -->



## In Summary - What is the difference between Runtime and compile time. 

I hope now after this you have a better understanding of compiler time and run time. What happens under the hood. 
