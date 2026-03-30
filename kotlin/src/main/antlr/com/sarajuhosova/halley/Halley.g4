grammar Halley;

program : stmt* expr EOF;

stmt
    : Let Name '=' expr                     #let
    | Name '=' expr                         #assign
    ;

expr
    : constant                              #const
    | Name                                  #var
    | op=UnOp e=expr                        #unop
    | left=expr op=BinOp right=expr         #binop
    ;

constant
    : Bool                                  #bool
    | Int                                   #int
    | String                                #string
    ;

EOL : ';';

Let : 'val' | 'var';

UnOp : '-' | '!';
BinOp
    : '=='
    | '<' | '<=' | '>' | '>='
    | '&&' | '||'
    | '+' | '-' | '*' | '/' | '%';

Bool : 'true' | 'false';
Int : '-'?Digit+;
String : '"' ( '\\"' | . )*? '"';

Name : (AlphaNum | '_')+ ;

AlphaNum
 :  'a'..'z'
 |  'A'..'Z'
 |  Digit
 ;

Digit : '0'..'9';

Comment : '/*' .*? '*/' -> skip;
InlineComment : '//' .*? ('\r'? '\n' | '\n' | EOF) -> skip;

Space :  [ \t\f]+ -> skip;
LineBreak: [\r\n] -> skip;
