# Key Concepts of TypeScript

TypeScript is a superset of JavaScript that adds static typing to the language. This means you can define the types of variables, function parameters, and return values, allowing the compiler to catch type-related errors during development rather than at runtime. This leads to more robust and maintainable code.

Here's a breakdown of the key concepts in TypeScript:

## 1. Static Typing

-   **Type Annotations:** You can explicitly specify the type of a variable using a colon (`:`) followed by the type.
    ```typescript
    let age: number = 30;
    let name: string = "Alice";
    let isStudent: boolean = true;
    ```

-   **Type Inference:** TypeScript can often infer the type of a variable based on its initial value, so you don't always need to write type annotations.
    ```typescript
    let city = "New York"; // TypeScript infers that 'city' is a string
    ```

-   **Primitive Types:** TypeScript supports the standard JavaScript primitive types:
    -   `number`: Represents numeric values (integers and floating-point numbers).
    -   `string`: Represents text.
    -   `boolean`: Represents true/false values.
    -   `null`: Represents the intentional absence of a value.
    -   `undefined`: Represents a variable that has not been assigned a value.
    -   `symbol`: Represents unique identifiers.
    -   `bigint`: Represents large integers.

- **Special Types**
    - `any`: Disables type checking. Use sparingly.
    - `unknown`: Similar to `any`, but safer. Requires type checking before use.
    - `void`: Represents the absence of a return value in a function.
    - `never`: Represents values that never occur (e.g., a function that always throws an error).

## 2. Interfaces

-   **Defining Object Shapes:** Interfaces define the structure of objects, specifying the names and types of their properties.
    ```typescript
    interface Person {
        name: string;
        age: number;
        email?: string; // Optional property
    }

    let user: Person = {
        name: "Bob",
        age: 25,
    };
    ```

-   **Extending Interfaces:** Interfaces can extend other interfaces, inheriting their properties.
    ```typescript
    interface Employee extends Person {
        employeeId: number;
    }
    ```

## 3. Classes

-   **Object-Oriented Programming:** TypeScript supports classes, which are blueprints for creating objects.
    ```typescript
    class Animal {
        name: string;

        constructor(name: string) {
            this.name = name;
        }

        makeSound() {
            console.log("Generic animal sound");
        }
    }

    let dog = new Animal("Buddy");
    dog.makeSound();
    ```

-   **Access Modifiers:** TypeScript provides access modifiers to control the visibility of class members:
    -   `public` (default): Accessible from anywhere.
    -   `private`: Accessible only within the class.
    -   `protected`: Accessible within the class and its subclasses.

- **Inheritance**
    - Classes can extend other classes, inheriting their properties and methods.

## 4. Functions

-   **Type Annotations for Parameters and Return Values:** You can specify the types of function parameters and the return value.
    ```typescript
    function add(a: number, b: number): number {
        return a + b;
    }
    ```

-   **Optional and Default Parameters:** You can make parameters optional or provide default values.
    ```typescript
    function greet(name: string, greeting?: string): void {
        console.log(`${greeting || "Hello"}, ${name}!`);
    }
    ```

- **Rest Parameters**
    - Allows a function to accept a variable number of arguments as an array.

## 5. Generics

-   **Reusable Components:** Generics allow you to create components that can work with a variety of types without sacrificing type safety.
    ```typescript
    function identity<T>(arg: T): T {
        return arg;
    }

    let myString = identity<string>("hello");
    let myNumber = identity<number>(123);
    ```

## 6. Enums

-   **Named Constants:** Enums provide a way to define a set of named constants.
    ```typescript
    enum Color {
        Red,
        Green,
        Blue,
    }

    let myColor: Color = Color.Green;
    ```

## 7. Type Aliases

- **Creating custom types**
    - Allows you to create a new name for an existing type.
    ```typescript
    type Point = {
        x: number;
        y: number;
    }
    ```

## 8. Union and Intersection Types

- **Union Types**
    - Allows a variable to hold values of multiple types.
    ```typescript
    let id: number | string;
    ```
- **Intersection Types**
    - Combines multiple types into one.
    ```typescript
    type Draggable = {
        drag: () => void;
    }

    type Resizable = {
        resize: () => void;
    }

    type UIWidget = Draggable & Resizable;
    ```

## 9. Modules

-   **Organizing Code:** Modules help organize code into separate files, improving maintainability.
-   **Exporting and Importing:** You can export types, functions, and classes from a module and import them into other modules.

## 10. Decorators

- **Metadata**
    - Decorators are a way to add metadata to classes, methods, properties, or parameters.

## Benefits of TypeScript

-   **Early Error Detection:** Catches type-related errors during development.
-   **Improved Code Readability:** Type annotations make code easier to understand.
-   **Better Code Maintainability:** Easier to refactor and maintain large codebases.
-   **Enhanced Tooling:** IDEs provide better autocompletion, refactoring, and navigation.
-   **Gradual Adoption:** You can gradually introduce TypeScript into an existing JavaScript project.
