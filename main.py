FILEPATH = "test.txt"
CODE = """
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
"""

f = open(FILEPATH, "a")
for c in CODE:
    if c == ">":
        f.write("   ")
    if c == "<":
        f.write("  \t")
    if c == "+":
        f.write(" \t ")
    if c == "-":
        f.write(" \t\t")
    if c == ".":
        f.write("\t  ")
    if c == ",":
        f.write("\t \t")
    if c == "[":
        f.write("\t\t ")
    if c == "]":
        f.write("\t\t\t")