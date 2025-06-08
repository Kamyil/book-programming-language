# Book language

We're striving for programming language, where it's code can be finally read "like a book" with little to no mental-overhead. Which is a huge challenge, because 
experienced programmers are used to the things working the way they were always working, while beginners scratch their heads 
wondering `What is this magic?`, `what is constuctor?` etc.. Also some languages contain very advanced features which tend to be overused, leading to confusion 
in large codebases. Main goal of this language is to make code *ACTUALLY* readable, by having extremely simple syntax which should make your codebase
simple and readable for both beginners (to the language itself and beginner programmers) but it should also be readable for large codebases out of the box

So the book language strives for followign principles:
- Readability
- Simplicity 
- Little to no mental-overhead
- "One, clear and obvious way of doing things" 
- Explicit (never implicit)
- Composition over inheritance

# Casing 
All `.book` code uses `snake_case` convention consistently, to make it easier to read the code for everyone. All other cases will be marked as errors
in both compiler and LSP, forcing programmer to follow the convention strictly (which should lead to code consistency)

# Files
Files have a `.book` extension. The programs starts by looking into `start.book` file and running the `start` function

```
call start
 // this is your entrypoint
end
```


# Data types
In this language you have data types that can be commonly found in other languages


```
2 // number (integer)
2.2 // number (float)
"String" // string
true or false // booleans
nothing // undefined, null and empty values
array // arrays
object  // objects
```


# Variables 
Variables can be declared only one way. The Beatles way. 
```
let it be 'something'
```

```
let this_thing be 1
let that_thing be 2

```

Using `let`, then following with variable name and equal being simply "be".
You can mutate (assign new values) to variables by using `becomes` keyword

```
let difficulty be "hard"
call print with difficulty // returns "hard"

difficulty becomes "simple"
call print with difficulty // returns "simple"

call print with time // returns 'simple'```

# Constants
Constants can be declared in similar way, but instead of `let`, they're declared with `constant` keyword followed by `is` keyword

```
constant SIZE is 2
```

Since it's truly a constant, you can't either mutate it or change the value via functions or methods

# Logical operators
Book language doesn't use `==`, `&&`, `||` `!`, `<`, `>` or other symbols for marking logic. 
It simply uses words like `is`, `and`, `or`, `not`, `less than` and `greather than`
So we can write complicated logic like this `if age = 18 && age < 24 == false` as 
```
if age is greater than 18 and is not less than 21
 call print with "You can drink alcohol almost anywhere!"
end
```


# Numbers
Numbers have a simple `Number` type which indicate if data is number. The type also has a subtype declaring if it's integer or float,
making it easier to distinguish between two. You can extract that subtype information by using global `is_integer` and `is_float` global helper functions

# Strings
Strings have only one way of declaration by using double-quotes "". The strings can contain multiple lines
making it easier to declare a longer strings. You can interpolate some variables directly into string with `{}` quirly brackets in the given position
without any whack-a-moles
```
let notification be "You have succesfully subscribed to {email_subscription_name}"
```


# Arrays
You can declare an array directly by using `array of` keyword and follow it with starting values
```

let books be array of 'Harry Potter', 'Lord Of The rings'

```
Following code will inherit the type to `Array of Strings`

## Ranges
You can also declare a range of values using "up to"
```
let pages be array of 1 up to 200
```

# Objects
You can declare an object directly by using `object with` keyword

```
let book be object with
  let title be "Harry Potter"
  let pages be 1 up to 200
end
```


Object can contain both fields and methods, that can be declared the same way as variables and functions, because they're simply are variables and functions.
Which should also make declaring less confusing, because of "one way of doing things" principle

```
let book be object with
  let is_read be false
  let title be "Harry Potter"
  let pages be 1 up to 200
  
  function mark_as_read takes new_is_read_value
    is_read becomes new_is_read_value
  end 
end
 

```


# Conditionals
You can declare a conditional using simple `if` / `else if` / `else ` blocks

```
let age be 17

if age is less than 18 then
  call print with "You cant drink alcohol in EU"
else if age is 18
  call print with "You can now drink alcohol in EU"
else
  call print with "You can drink alcohol in EU for quite some time"
end 

```


# Loops
You can do loop... by simply writing a `loop`
```
loop
 // .. your code
end

```

you can declare the condition when you want to break

```
let input_value be 2

loop
  input_value becomes 3

  if input_value is 10 then break
end
```


You can also loop through items using `foreach` loop
```
let items be array of 1, 2, 3, 4

foreach item in items
    call print with item
end

```


# Types
Book language is statically typed for consistency, but it also inferences those types making your code still consitent, predictable and reliable (no matter of scale)
without unneeded verbosity. In most places types are required, but there are some places where you need to be explicit of what you mean.
Treat this language as a book and give readers your intention. You can declare a type containing of the values by using `is either` and `or` keywords

```
type Flavour is either 'chocolate' or 'vanilla' or 'strawberry'
```
or you can use other types to define other type

```
type Flavours is array of 'Chocolate', 'Vanilla' or 'Strawberry'
```

and you can combine both types and declare single thing of other type using `one of` keyword

```
type Flavour is one of Flavours
```



# Functions
You can declare a function using `function` keyword. 
You can declare arguments with `takes` keyword
You can return a value using `returns` keyword
and you can call functions using `call` keyword `with` your arguments (using `with` keyword)

(look that we're using Present Simple here, because we clearly dinstinct "declaration phase" and "call to action" phase, making it clear for beginners
where we're using the stuff and where do we setup the stuff)

```
function add takes number1, number2 
  returns number1 + number2
end 

call add with 2,3 // returns 5

```


# Recipes (Classes) 
In this language, we call classes a "recipes", because in most (if not all) languages, classes are simply a recipes for creating objects. 
So we call them `recipes` directly.  
You can declare such recipe using `recipe keyword`

```
recipe Cake 
  let ingredients be array of 'Butter', 'Milk', 'Eggs', 'Strawberries'
end 

```
For more simplicity, fields and methods are declared the same way as variables and functions, because... they're simply are variables and functions
Methods (recipe's functions) are public by default, but you can make it private by using `private` function

```
recipe Cake 
  let ingredients be array of Ingredients
  
  private function 
end 

```

Recipes don't require writing a constructor, because you can assign the values directly using `where` keyword


```
let my_first_cake be new Cake where flavour is 'chocolate'

```
Where there are multiple fields being assigned during creation you can split it into multiple lines

```
let my_first_cake be new Cake where 
    flavour is 'chocolate'
    

```
recipes don't really have the concept of constructors because they don't need it. 
But ofc. you can hook into object creation when needed by using `before_create`
```

recipe Cake 
   flavour is one of Flavours
   ingredients are array of Ingredients

   before_create
     call check_ingredients
   end
end 
```

We don't really use `on` keyword for hooking into events, because 'on' doesn't clearly indicate
if given hook is called before the event or after the event. I know it may sound weird at first and "after the event" seems as obvious answer for that but 
consider following code `<Modal onOpen={}` ... does `onOpen` call before after the event? If so, then is it really handful if we want to 
pass some data into modal before opening it? So if it's `before` then how can we call something after opening the modal like some polishing function
Book language doesn't have that confusion, because we're clearly indicating if some events are happening `before` or `after`




# Modules
We don't have a 'relative' paths approach, but we neither have a complicated Rust approach for declaring modules.
Book always look from top  of root directory structure, so importing from there is simply passing a file name.
For nested files inside nested directories you can simply follow it's path with slashes `/`

Considering follwing file structure
```
- nested_folder
    - nested_module.book
- module_a.book
- start.book
```

In `start.book` file we can import external modules like this

```
get everything as module_a from 'module_a'
get everything as nested_module from 'nested_folder/nested_module'

call start
  call module_a.start with 'Hi mom!'
end
```

You can either `get everything` (both are keywords) from given module or get single items from given module

```
get some_variable from 'module_a'
```

However, all things within a module are private unless you use `shared` keyword to explicity share them with others 

```

shared let thing be 2
```

<!-- TODO: Still don't know whetver "all things public by default" are better or "all things private by default"  -->


