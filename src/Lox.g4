grammar Lox;
program: declaration* EOF;
declaration:
	varDecl			# variable
	| statement		# statment
	| functionDecl	# function;

functionDecl: 'fun' id = IDENTIFIER '()' body = block;

varDecl: 'var' IDENTIFIER ( '=' expr = expression)? ';';
statement:
	exprStmt
	| ifStmt
	| printStmt
	| whileStmt
	| forStmt
	| breakStmt
	| block;

block: LCURL declaration* RCURL;
breakStmt: 'break' ';';
whileStmt:
	'while' '(' condition = logic_or ')' body = statement;

forStmt:
	'for' '(' (initializer = exprStmt | forvar = varDecl | ';') (
		condition = logic_or
	)? ';' (increment = expression)? ')' body = statement;
exprStmt: expression ';';
printStmt: 'print' exp = expression ';';
ifStmt:
	'if' '(' condition = logic_or ')' thenBranch = statement (
		'else' elseBranch = statement
	)?;
expression: assignment;

assignment:
	IDENTIFIER '=' iter = assignment	# assignment_alt
	| logic_or							# logic_or_alt;

logic_or: left = logic_and ( 'or' right = logic_and)?;
logic_and: left = equality ( 'and' right = equality)?;
equality: left = comparison ( ( NEQ | EQ) right = comparison)?;
comparison: left = term ( ( GT | GTE | LT | LTE) right = term)?;

term: left = factor ( ( MINUS | PLUS) right = factor)*;
factor: left = unary ( ( SLASH | STAR) right = unary)*;

unary: (BANG | MINUS) right = unary	# unary_alt
	| primary						# primary_alt;

// callfun: primary '(' arguments? ')'; arguments: expression ( ',' expression)*;
primary:
	'true'					# bool_true
	| 'false'				# bool_false
	| 'nil'					# nil
	| NUMBER				# number
	| STRING				# strval
	| IDENTIFIER			# identifier
	| '(' expression ')'	# group;

LCURL: '{';
RCURL: '}';
BANG: '!';
MINUS: '-';
SLASH: '/';
STAR: '*';
PLUS: '+';
GT: '>';
GTE: '>=';
LT: '<';
LTE: '<=';
EQ: '==';
NEQ: '!=';
WS: [ \t\r\n]+ -> skip; //channel(HIDDEN);
NUMBER: DIGIT+ ( '.' DIGIT+)?;
STRING: '"' .*? '"';
IDENTIFIER: ALPHA ( ALPHA | DIGIT)*;
ALPHA: 'a' .. 'z' | 'A' .. 'Z' | '_';
DIGIT: '0' .. '9';
LINE_COMMENT:
	'//' .*? '\r'? '\n' -> skip; // Match "//" stuff '\n'
COMMENT: '/*' .*? '*/' -> skip; // Match "/*" stuff "*/"