fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    let faa: Foo = Foo { x: (1, 2), y: 3 };

    // You do not need a match block to destructure structs:
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    // Destructuring works with nested structs as well:
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}


/*
What Happens to y in This Pattern?
In this pattern:

Point { x, y: 0 }

Here's the breakdown:

- x is a binding. It means: "Whatever the value of p.x is, bind it to a variable named x so I can use it."

- y: 0 is a match against a literal. It means: "Only match this arm if the p.y field is exactly 0."

So, to directly answer your question:

What happens to y?

p.y is compared to 0.

No variable is bound to the value of p.y.

The matcher uses the value of p.y during the comparison, but that value is not saved or made available in the match arm’s body, because you didn't bind it.

In short:

x is stored in a variable named x for use in the match arm.

y is not stored or made available — it's just compared to 0, and then discarded.




Rust Pattern Matching Binding Rules
By default, when you write something like x or y in a pattern (e.g., Point { x, y }), Rust binds the corresponding field's value to a new variable with that name.

But if you specify a literal or a pattern with a value, like y: 0, Rust understands that as a value to match, not a binding.

In Your Example

match p {
    Point { x, y: 0 } => println!("On the x axis at {x}"),
    Point { x, y } => println!("On neither axis: ({x}, {y})"),
}
x is bound → x = p.x

y: 0 means "match only if p.y == 0", but don't bind p.y to anything.

So:

You can use x in the first arm.

You cannot use y in the first arm, because there's no variable y bound — just a condition p.y == 0.


Yes — the matching pattern does destructure the Point struct p.

What does destructuring mean?
Destructuring means breaking a complex value (like a struct or tuple) into its individual components (fields or elements), so you can:

Match specific values (e.g., y: 0)

Or bind parts to variables (e.g., x or y)

Your example again:

struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

match p {
    Point { x, y: 0 } => println!("On the x axis at {x}"),
    Point { x, y } => println!("On neither axis: ({x}, {y})"),
}
What happens step by step:
match p — you're matching against the value of p, which is a Point { x: 0, y: 7 }.

Rust tries to match the value to the first arm: Point { x, y: 0 }

This destructures p into its two fields, x and y.

It binds x to p.x (0).

It compares p.y to 0. In this case, it's not equal (p.y == 7), so the match fails.

Rust moves to the second arm: Point { x, y }

This also destructures p into x and y.

Both x and y are bound to p.x and p.y, respectively.

This match succeeds, and x == 0, y == 7 are now available for use.

So yes:
✅ Both arms destructure p
❌ But only the second one binds both x and y
✅ The first one binds x, and uses y in a literal match




Let’s walk through a visual breakdown of what happens internally when Rust evaluates your match expression. This will show how p is destructured, and how each field is either bound to a variable or used for a value comparison.

🧱 Struct Definition

struct Point {
    x: i32,
    y: i32,
}
📦 Value

let p = Point { x: 0, y: 7 };
So p holds this:

yaml
Copy
p = Point {
    x: 0,
    y: 7
}
🎯 Match Statement

match p {
    Point { x, y: 0 } => println!("On the x axis at {x}"),
    Point { x, y } => println!("On neither axis: ({x}, {y})"),
}
Let’s go arm-by-arm.

✅ Arm 1: Point { x, y: 0 }
🔍 What Rust does:
Destructure p into its fields:

p.x = 0

p.y = 7

Pattern Matching:

x → bind p.x = 0 to variable x

y: 0 → check if p.y == 0

❌ It’s not — p.y == 7, so this pattern does not match

Result: This arm is skipped. Only x would be bound, but y is not a variable — it was used for comparison.

✅ Arm 2: Point { x, y }
🔍 What Rust does:
Destructure p:

p.x = 0

p.y = 7

Pattern Matching:

x → bind p.x = 0 to variable x

y → bind p.y = 7 to variable y

Result: This pattern matches, and now:

ini
Copy
x = 0
y = 7
✅ You can use both x and y in the match arm body.

🧠 Summary Diagram

match p {
    Point { x, y: 0 } => {
        // Destructures p
        // x = p.x
        // compares p.y == 0
        // ✅ binds x
        // ❌ y not bound
    }

    Point { x, y } => {
        // Destructures p
        // x = p.x
        // y = p.y
        // ✅ binds x and y
    }
}

*/




struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p: Point = Point { x: 0 , y: 10 };

    match p {
        /*
        In Rust, pattern matching is used to destructure data and optionally match literal values.

        Variable Binding: Captures the value of a component.

        match p {
            Point { x, y } => println!("x is {x} and y is {y}"),
        }
        Here, x and y are variable bindings that capture the values of p.x and p.y.

        Literal Match: Checks if a component matches a specific value.


        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x, y } => println!("On neither axis: ({x}, {y})"),
        }
        Here, y: 0 checks if y is 0. It doesn't bind y as a variable.


        - `Point { x, y: 0 }` matches only if y is 0. x is bound to p.x.
        
        - `Point { x, y }` matches any Point, binding `x` to `p.x` and `y` to `p.y`.

        In Rust, y: 0 in pattern matching is a literal match, not a variable binding.
        In JavaScript, y: newY in destructuring is used to rename a property.

        The first arm will match any point that lies on the x axis by specifying that the y field matches if its value matches the literal 0. The pattern still creates an x variable that we can use in the code for this arm.

        We can also destructure with literal values as part of the struct pattern rather than creating variables for all the fields. Doing so allows us to test some of the fields for particular values while creating variables to destructure the other fields.


        Key Points
        Literal vs. Variable Binding:
        Point { x, y: 0 } is checking if the y field of Point is exactly 0. This is a literal match.
        Point { x, y } is binding the value of the y field to a variable named y.
        
        Purpose of Literal Matching:
        When you specify y: 0, you are not interested in capturing the value of y; you are asserting that y must be 0 for the pattern to match.
        Since you are not capturing y (you're merely checking it), the language does not create a binding for y.
        Language Design Philosophy:

        Rust's pattern matching syntax distinguishes between matching values and binding variables for clarity and to avoid ambiguity.
        Allowing the use of y in the arm where y: 0 could create confusion about whether y is a variable or a value.


        Here, `x` and `y` in the pattern (`Point { x , y }`) are placeholders or variables that we are creating to bind/assign the values of the `x` and `y` fields from the `Point` struct `p` Specifically:
            - `x`: This variable is being bound to the value of the `x` field of the `Point` struct `p` that
            matches the pattern. In other words, `x` will hold the value of `p.x`.
            - `y`: Similarly, this variable is being bound to the value of the `y` field of the Point struct p that matches the pattern. So, `y` will hold the value of `p.y`.

        Point { x, y } => println!("x: {}, y: {}", x, y)

        In the match arm `Point { x: 0, y }`, `y` is being bound to the value of the `y` field of the `Point` struct `p`. However, `x` is not being directly used in this arm, it's only being pattern matched against the value `0` – `0` is not bound to `x`.

        the variables `x` and `y` created within the match arm to hold the values extracted from the `Point` struct `p`. These bindings are only valid within the scope of that match arm.
        */
        Point { x: 0 , y } => println!("2. On the a axis at and b axis at {} and {}", p.x, y),

        /*
        x: a: Here, we are matching the x field of the Point struct p with a new variable a. So, a is bound to the value of the x field of p.
        y: b: Similarly, we are matching the y field of the Point struct p with a new variable b. b is bound to the value of the y field of p.
        Then, we use these newly bound variables a and b within the println! macro to print the values.

        So, in this notation:

        a represents the value of the x field of p.
        b represents the value of the y field of p.
        These bindings are specific to this match arm and are only valid within its scope. They are separate from the fields x and y declared in the Point struct.
        */
        Point { x , y } => println!("3. On the a axis at and {} b axis at {}", x, y),

    }
}