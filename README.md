# another programming language

This time it is a simple language with conditions, functions, basic arithmetic, and variables

```
function	::= fn identifier '(' args ')' body

args		::= variable (',' variable)*

body		::= '{' (statement)+ '}'

statement	::= assignment | condition | return

assignment	::= identifier '=' expression ';'

condition	::= 'if' expression body ( 'else' 'if' expression body )* ( 'else' body )?

return		::= 'return' expression ';'

expression	::= comparison

comparsion	::= factor ( '==' | '>' | '>=' | '<' | '<=' factor)*

factor		::= term ( '*' | '/' term )*

term		::= literal ( '+' | '-' literal )*

literal		::= '(' expression ')' | number | variable | call

number		::= '+' | '-' [0-9]+ ( '.' [0-9]+ )?

variable	::= identifier

call		::= identifier '(' expression ( ',' expression )* ')'

identifier	::= [_a-zA-Z]+[a-zA-z0-9]*
```
