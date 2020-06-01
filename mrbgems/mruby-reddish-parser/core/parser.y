%{
#include "mruby.h"
#include "mruby/string.h"
#include "mruby/array.h"
#include "mruby/variable.h"

typedef struct parser_state {
    mrb_state* state;
    mrb_value  parser_class;
    mrb_value  result;
    mrb_value  action_class;
} parser_state;

#define ACTION(p, n, c, ...)   mrb_funcall(p->state, p->action_class, n, c, __VA_ARGS__)
#define MRB_CONST_SET(s, c, v) mrb_const_set( s, \
                                mrb_obj_value(c), \
                                mrb_intern_lit(s, #v), \
                                mrb_fixnum_value((mrb_int)v) )

#define YYDEBUG 1

static int yylex(void* , parser_state*);
static void yyerror(parser_state*, const char*);
static int yyparse(parser_state*);
%}

%define api.value.type {mrb_value}
%define api.pure
%parse-param {parser_state* p}
%lex-param {parser_state* p}

%token WORD NUMBER MINUS NUMBER_MINUS
%token AND AND_AND OR OR_OR
%token GT GT_GT LT
%type wordlist
%type simple_command simple_list
%start inputunit

%left AND_AND OR_OR

%%

inputunit
: %empty
| simple_list { p->result = $1; }

simple_list
: simple_command                     { $$ = $1; }
| simple_list AND_AND simple_command { $$ = ACTION(p, "make_and_command_connector", 2, $1, $3); }
| simple_list OR_OR   simple_command { $$ = ACTION(p, "make_or_command_connector",  2, $1, $3); }

simple_command
: wordlist { $$ = ACTION(p, "make_command", 1, $1); }
/* <    */| simple_command LT wordlist          { $$ = ACTION(p, "assign_read_redirect", 2, $1, $3); }
/* n<   */| simple_command NUMBER LT wordlist   { $$ = ACTION(p, "assign_read_redirect", 3, $1, $4, $2); }
/* <&-  */| simple_command LT AND MINUS         { $$ = ACTION(p, "assign_close_redirect", 2, $1, mrb_fixnum_value(0)); }
/* <&n  */| simple_command LT AND NUMBER        { $$ = ACTION(p, "assign_copy_read_redirect", 2, $1, $4); }
/* <&n- */| simple_command LT AND NUMBER_MINUS  { $$ = ACTION(p, "assign_copy_write_redirect", 3, $1, $4, mrb_fixnum_value(0));
                                                       ACTION(p, "assign_close_redirect", 2, $1, $4); }
/* n<&- */| simple_command NUMBER LT AND MINUS  { $$ = ACTION(p, "assign_close_redirect", 2, $1, $2); }
/* n<&n */| simple_command NUMBER LT AND NUMBER { $$ = ACTION(p, "assign_copy_read_redirect", 3, $1, $5, $2); }
/* n<&n-*/| simple_command NUMBER LT AND NUMBER_MINUS { $$ = ACTION(p, "assign_copy_write_redirect", 3, $1, $5, $2);
                                                             ACTION(p, "assign_close_redirect", 2, $1, $5); }

/* >    */| simple_command GT wordlist          { $$ = ACTION(p, "assign_write_redirect",2, $1, $3); }
/* n>   */| simple_command NUMBER GT wordlist   { $$ = ACTION(p, "assign_write_redirect", 3, $1, $4, $2); }
/* >&-  */| simple_command GT AND MINUS         { $$ = ACTION(p, "assign_close_redirect", 2, $1, mrb_fixnum_value(1)); }
/* >&n  */| simple_command GT AND NUMBER        { $$ = ACTION(p, "assign_copy_write_redirect", 2, $1, $4); }
/* >&n- */| simple_command GT AND NUMBER_MINUS  { $$ = ACTION(p, "assign_copy_write_redirect", 3, $1, $4, mrb_fixnum_value(1));
                                                       ACTION(p, "assign_close_redirect", 2, $1, $4); }
/* n>&- */| simple_command NUMBER GT AND MINUS  { $$ = ACTION(p, "assign_close_redirect", 2, $1, $2); }
/* n>&n */| simple_command NUMBER GT AND NUMBER { $$ = ACTION(p, "assign_copy_write_redirect", 3, $1, $5, $2); }
/* n>&n-*/| simple_command NUMBER GT AND NUMBER_MINUS { $$ = ACTION(p, "assign_copy_write_redirect", 3, $1, $5, $2);
                                                             ACTION(p, "assign_close_redirect", 2, $1, $5); }
/* &>   */| simple_command AND GT wordlist       { $$ = ACTION(p, "assign_write_redirect", 2, $1, $4);
                                                        ACTION(p, "assign_copy_write_redirect", 3, $1, mrb_fixnum_value(1), mrb_fixnum_value(2)); }
/* >&   */| simple_command GT AND wordlist       { $$ = ACTION(p, "assign_write_redirect", 2, $1, $4);
                                                        ACTION(p, "assign_copy_write_redirect", 3, $1, mrb_fixnum_value(1), mrb_fixnum_value(2)); }

/* >>   */| simple_command GT_GT wordlist        { $$ = ACTION(p, "assign_append_redirect", 2, $1, $3); }
/* n>>  */| simple_command NUMBER GT_GT wordlist { $$ = ACTION(p, "assign_append_redirect", 3, $1, $4, $2); }
/* <>   */| simple_command LT GT wordlist        { $$ = ACTION(p, "assign_read_write_redirect", 2, $1, $4); }
/* n<>  */| simple_command NUMBER LT GT wordlist { $$ = ACTION(p, "assign_read_write_redirect", 3, $1, $5, $2); }

wordlist
: WORD          { $$ = ACTION(p, "make_word_list", 1, $1); }
| wordlist WORD { $$ = ACTION(p, "add_to_word_list", 2, $1, $2); }

%%

int yylex(void* lval, parser_state* p) {
    mrb_value token;
    int type;

    token = mrb_funcall(p->state, p->parser_class, "get_token", 0);

    if (mrb_nil_p(token)) {
        return YYEOF;
    }

    *((YYSTYPE*)lval) = mrb_funcall(p->state, token, "word", 0);
    type = mrb_fixnum(mrb_funcall(p->state, token, "type", 0));

    return type;
}

void yyerror(parser_state* p, const char* s){
    mrb_value str = mrb_str_new_cstr(p->state, s);
    mrb_funcall(p->state, p->parser_class, "error", 1, str);
}

mrb_value mrb_reddish_parser_parse(mrb_state *mrb, mrb_value self) {
    mrb_value inputline;
    parser_state pstate;
    struct RClass* rp = mrb_module_get(mrb, "ReddishParser");

    mrb_get_args(mrb, "S", &inputline);

    pstate.state = mrb;
    pstate.parser_class = mrb_obj_new(mrb, mrb_class_get_under(mrb, rp, "Parser"), 1, &inputline);
    pstate.result = mrb_nil_value();
    pstate.action_class = mrb_obj_value(mrb_class_get_under(mrb, rp, "Action"));

    yyparse(&pstate);

    if (mrb_nil_p(pstate.result)) {
        return mrb_nil_value();
    }

    return pstate.result;
}

mrb_value mrb_reddish_parser_debug(mrb_state *mrb, mrb_value self) {
    mrb_bool flag;
    mrb_get_args(mrb, "b", &flag);
    yydebug = flag;
    return mrb_bool_value(flag);
}

void mrb_tokentype_initialize(mrb_state* mrb, struct RClass* tt) {
    MRB_CONST_SET(mrb, tt, YYEOF);
    MRB_CONST_SET(mrb, tt, WORD);
    MRB_CONST_SET(mrb, tt, NUMBER);
    MRB_CONST_SET(mrb, tt, MINUS);
    MRB_CONST_SET(mrb, tt, NUMBER_MINUS);
    MRB_CONST_SET(mrb, tt, AND);
    MRB_CONST_SET(mrb, tt, AND_AND);
    MRB_CONST_SET(mrb, tt, OR);
    MRB_CONST_SET(mrb, tt, OR_OR);
    MRB_CONST_SET(mrb, tt, GT);
    MRB_CONST_SET(mrb, tt, GT_GT);
    MRB_CONST_SET(mrb, tt, LT);
}
