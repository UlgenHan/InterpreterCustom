# Abstract
This project report details the implementation of a domain-specific language
(DSL) using the Rust programming language. The project involves the development of a Rust-based interpreter for a simplified programming language.
The interpreter, constructed step-by-step, demonstrates the fundamental processes of lexical analysis, parsing, and code execution using abstract syntax
trees (ASTs). From tokenization to the handling of variables and functions,
the report illustrates how Rust’s language features empower the creation of a
robust interpreter. Through practical examples and insights, this project illuminates the journey from language specification to a functional interpreter,
offering valuable lessons in systems programming and language implementation.

## Introduction
In our previous report, we explored the Rust programming language, its unique
features, and its growing popularity in the technology community. Building
on this foundation, we embarked on an ambitious project to create a domainspecific language (DSL) using Rust. This report documents the development of
this DSL, providing a comprehensive overview of the implementation process,
challenges encountered, and solutions devised.

### Example of designed language

variable integer x = 4;
variable integer y = 8;

function integer calculate(variable integer param1, variable integer param2){
            return param1 + param2;
}
        
variable integer result = call calculate(x, y);
print result;

## Design and Implementation

### Lexical Analysis (Lexing) and Tokenization
The first step in creating our DSL was lexical analysis, where the source code is
converted into a sequence of tokens. Tokens are the basic building blocks of the
language, representing keywords, identifiers, operators, and other symbols. We
implemented a lexer in Rust to scan the input source code and generate tokens.

![image](https://github.com/UlgenHan/InterpreterCustom/assets/160638676/51f61bb5-fe1a-475c-9eb9-c566620113f8)

## Parsing Techniques and Abstract Syntax Trees (ASTs)
After tokenization, the next step was parsing, where tokens are transformed
into a structured representation called an abstract syntax tree (AST). The AST represents the grammatical structure of the source code and is used for further
processing, such as code generation and execution.

![image](https://github.com/UlgenHan/InterpreterCustom/assets/160638676/80f0d708-32c6-4404-8410-32af772a2430)


## Code Evaluation and Execution

The final step involved evaluating the AST to execute the code. This step
involves interpreting the AST nodes, managing variable assignments, function
calls, and control structures

![image](https://github.com/UlgenHan/InterpreterCustom/assets/160638676/6bd46571-2b06-466d-b77a-9b5581a76706)

## Utilization of Rust Language Features

Rust’s features such as pattern matching, ownership, and lifetimes played a crucial role in the implementation of our DSL. These features helped in managing
memory safely and efficiently, ensuring the robustness of the interpreter.


