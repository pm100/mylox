grammar Lox;
program: declaration* EOF;
declaration: varDecl # variable | statement # statment;

varDecl: 'var' IDENTIFIER ( '=' expr = expression)? ';';
statement: exprStmt | printStmt;

exprStmt: expression ';';
printStmt: 'print' exp = expression ';';

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

primary:
	'true'					# bool_true
	| 'false'				# bool_false
	| 'nil'					# nil
	| NUMBER				# number
	| STRING				# strval
	| IDENTIFIER			# identifier
	| '(' expression ')'	# group;

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
WS: [ \t\r\n]+ -> channel(HIDDEN);
NUMBER: DIGIT+ ( '.' DIGIT+)?;
STRING: '"' .*? '"';
IDENTIFIER: ALPHA ( ALPHA | DIGIT)*;
ALPHA: 'a' .. 'z' | 'A' .. 'Z' | '_';
DIGIT: '0' .. '9';
LINE_COMMENT:
	'//' .*? '\r'? '\n' -> skip; // Match "//" stuff '\n'
COMMENT: '/*' .*? '*/' -> skip; // Match "/*" stuff "*/"