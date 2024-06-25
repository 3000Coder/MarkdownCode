# WIP MarkCode
Turing complete programming language inside markdown format.

# Language overview: 

## Types
number - 64bit floats
sting - string of characters

## Mics
\> - prints whatever is suplied

unmarked text is considered a comment

## Variables
In MarkCode variables are declared using link syntax.

\[value\]\(name\)

And their value can be rewritten using:

\!\[value\]\(name\)

Why not use \[name\]\(value\)?
Links are not formatted so using variables or expressions would be impossible. It also makes it impossible to insert variables and expressions into variable name which simplifies things a lot.

Note if used in expression variable asignments are converted to a value.

To acess value of variable use:

\*\*hello world\*\*

## Flow control:
MarkCode code is processed by splitting the program line by line and running the code top to bottom. To jump around markers are used.

You can declare marker using like this:

\__test__

Now that we have declared marker we can jump to it like this:

\`test\`

Now that we have jumped to a mark code execution should continue flow mark like normal.

### If statement
To use if statements use \# and close with \-\-\-.

MarkCode doesn't support booleans, you can use integers instead. Every number that isn't 0 evaluates to true. All strings are evaluated to true.

### While loop
To use while loop use \## and close with \-\-\-.

## Arrays
Note that in MarkCode arrays are always statically sized and imutable.

You can declare array like this:

\#\#\#\# Array name
- 1
- 2
- 3

## Functions
You can declare function like this:

\#\#\# Function name
1. argument1
2. argument2
3. argument3

Now you can use function arguments as any other variables. Don't forget to close the scope using \-\-\-.
