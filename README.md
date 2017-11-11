_mex_ a parser/evaluator for simple mathematical expressions, with _no_ dependencies at all. Currently, it supports only a _tiny_ amount of syntax. Here are some valid expressions:

```
>> 10
10

>> 10 - 3 * 10
-20

>> π = 3.1415926536
3.1415926536

>> 5 * π
15.707963268
```

And that's it, for now.

It supports the following operators: `+`, `-`, `*`, `/`, `=`.

But it's easy to add more (by editing the source).

The crate can be used as a binary or a library. The binary is a REPL, where you enter single expressions line by line and get the result given back to you. An example of the library is below:

```rust
use mex::parser;
use mex::evaluator;
use mex::evaluator::context;
use mex::evaluator::object;

fn main() {
    let input = "2 * x";
    let mut parse = parser::Parser::new(input);

    let mut context = context::Context::new();
    context.set(&String::from("x"), object::Object::Number(13.5));

    match parse.parse() {
        Ok(node) => {
            match evaluator::eval_node_in(node, &mut context) {
                Ok(result) => println!("{}", result),
                Err(err) => println!("eval error: {:?}", err),
            }
        }

        Err(e) => println!("parse error: {:?}", e),
    };
}
```

In the future, I'd like to add these things:

 - Simple API for simple uses
   - e.g. `mex::evaluate("1 + 2")` &rarr; `3`

 - Functions
   - e.g. `f(x, y) = x + y`
   - e.g. `f(2, 3)` &rarr; `5`
 
 - Customisation / options
   - Probably via bitflags
   - Things like which operators are allowed, if you're allowed to define variables, etc...

 - Builtins
   - e.g. `π`, `e`, `sin(θ)`
   - Going back to the last point: builtins should be optional

 - Multiple expressions in one
   - e.g. `5 + {1, 2, 3}` &rarr; `6`, `7`, and `8`
   - They would be returned in a hash set, probably

 - Symbolic Expressions
   - e.g. `'x = a + b'` (maybe with different delimiters)
   - Can be either equations (i.e. with equals), or just simple expressions
   - `where` clause
     - e.g. `'a + b' where a = 2, b = 10` &rarr; `12`
   - `solve ... for`
     - e.g. `solve 'a = x + b' for x` &rarr; `'x = a - b'`
     - e.g. `(solve 'a = x + b' for x) where a = 5, b = 2` &rarr; `3`
     - e.g. `solve 'x^2 + 3x + 2' for x` &rarr; `(x + 1)(x + 2)`
   - Maybe even integration/differentiation
