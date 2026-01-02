# another programming language

This time it is a simple language with conditions, functions, basic arithmetic, and variables

grammar
```
statement	::= simple_statment | compound_statement

simple_statement	::= assignment | return | print ';'
compound_statement	::= function | if

function	::= fn identifier '(' param (',' param)* ')' body
param		::= identifier

if			::= 'if' expression body ( 'else' 'if' expression body )* [ 'else' body ]

body		::= '{' (simple_statement | condition)+ '}'

assignment	::= identifier '=' expression
return		::= 'return' expression
print		::= 'print' expression

expression	::= comparison
comparsion	::= term ( '==' | '>' | '>=' | '<' | '<=' term)*
term		::= factor ( '+' | '-' factor )*
factor		::= unary ( '*' | '/' unary )*
exponent	::= unary '^' unary
unary		::= ('+' | '-') literal
literal		::= '(' expression ')' | number | variable | call
number		::= '+' | '-' [0-9]+ ( '.' [0-9]+ )?
variable	::= identifier
call		::= identifier '(' expression ( ',' expression )* ')'

identifier	::= [_a-zA-Z]+[a-zA-z0-9]*
```
love you so much

ast
```
stmt	::= if(expr *bool, stmt body[], stmt else_body[])
|	func(id name, id args[], stmt body[])
|	assign(id var, expr value)
|	return(expr value)
|	print(expr value)
|	expression(expr val)

expr	::= binary	(expr left, binaryop op, expr right) 
|	unary(unaryop op, expr right)
|	group(expr)
|	call(id name, expr args[])
|	number(f64)
|	bool(bool)
|	var(id name)

unaryop		::= ( 'return' | 'print' | '+' | '-')
binaryop	::= ( '+' | '-' | '*' | '/' | '^' | '=' | '==' | '>' | '>=' | '<' | '<=' )
id			::= identifier
```
