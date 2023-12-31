grammar MiniImp;

fragment Digit : '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' ;
fragment Letter : [A-Z] ;

Identifier : Letter ( Letter | Digit | '_' )* ;
Number : Digit Digit* ;
truth : 'true' | 'false' | 'not' truth | 'is' Identifier expr | truth 'or' truth | truth 'and' truth;

expr   : term (('+' | '-') term)* ;
term   : factor (('*' | '/') factor)* ;
factor : ('(' expr ')') | truth | Identifier | Number | STRING;

stmt   : select | iterat | set | write | read | asNumber | asString;
select : 'if' expr 'then' scope 'else' scope ;
iterat : 'while' expr scope ;
set    : 'set' Identifier '=' expr ';' ;
write  : 'write' expr ';' ;
read   : 'read' expr ';' ;

decl     : variable ;
variable : 'var' Identifier ( '=' expr )? ';' ;
asNumber : Identifier 'asNumber' ';';
asString : Identifier 'asString' ';';

stmts : stmt stmt* ;
decls : decl decl* ;
scope : 'begin' decls? stmts? 'end.' ;
init  : 'program' Identifier ;
prog  : init scope ;

WS : [ \t\n\r] + -> skip ;
STRING : '"' ~('"')* '"' ;
