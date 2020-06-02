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
#define REDIRECT(p, t, c, ...) ACTION(p, "make_redirect", (c+1), mrb_str_new_cstr(p->state, t), __VA_ARGS__)
#define CONNECTOR(p, t, c, ...)ACTION(p, "make_command_connector", (c+1), mrb_str_new_cstr(p->state, t), __VA_ARGS__)
#define FIXNUM(i) mrb_fixnum_value(i)
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
%token AND AND_AND OR OR_OR SEMICOLON
%token GT GT_GT LT
%start inputunit

%left AND_AND OR_OR

%%

inputunit
: %empty
| command_list { p->result = $1; }

command_list
: simple_command { $$ = ACTION(p, "make_command_list", 1, $1); }

simple_command
: simple_command_element
| simple_command AND_AND   simple_command_element { $$ = CONNECTOR(p, "AND", 2, $1, $3); }
| simple_command OR_OR     simple_command_element { $$ = CONNECTOR(p, "OR",  2, $1, $3); }
| simple_command SEMICOLON simple_command_element { $$ = CONNECTOR(p, "SEMICOLON", 2, $1, $3); }

simple_command_element
: wordlist                     { $$ = ACTION(p, "make_command", 1, $1); }
| simple_command_element redirect_list { $$ = ACTION(p, "assgin_redirect_list", 2, $1, $2); }

redirect_list
: redirect { $$ = ACTION(p, "make_redirect_list", 1, $1); }
| redirect_list redirect { $$ = ACTION(p, "add_redirect_list", 2, $1, $2); }

redirect
/* <    */: LT wordlist                { $$ = REDIRECT(p, "READ",     2, FIXNUM(0), $2); }
/* n<   */| NUMBER LT wordlist         { $$ = REDIRECT(p, "READ",     2, $1, $3); }
/* <&-  */| LT AND MINUS               { $$ = REDIRECT(p, "CLOSE",    1, FIXNUM(0)); }
/* <&n  */| LT AND NUMBER              { $$ = REDIRECT(p, "COPYREAD", 2, FIXNUM(0), $3); }
/* <&n- */| LT AND NUMBER_MINUS        { const mrb_value vals[] = {
                                            REDIRECT(p, "COPYREAD", 2, FIXNUM(0), $3),
                                            REDIRECT(p, "CLOSE",    1, $3) };
                                         $$ = mrb_ary_new_from_values(p->state, 2, vals);
                                       }
/* n<&- */| NUMBER LT AND MINUS        { $$ = REDIRECT(p, "CLOSE",    1, $1); }
/* n<&n */| NUMBER LT AND NUMBER       { $$ = REDIRECT(p, "COPYREAD", 2, $1, $4); }
/* n<&n-*/| NUMBER LT AND NUMBER_MINUS { const mrb_value vals[] = {
                                            REDIRECT(p, "COPYREAD", 2, $1, $4),
                                            REDIRECT(p, "CLOSE",    1, $4)    };
                                         $$ = mrb_ary_new_from_values(p->state, 2, vals);
                                       }

/* >    */| GT wordlist                { $$ = REDIRECT(p, "WRITE",     2, FIXNUM(1), $2); }
/* n>   */| NUMBER GT wordlist         { $$ = REDIRECT(p, "WRITE",     2, $1, $3); }
/* >&-  */| GT AND MINUS               { $$ = REDIRECT(p, "CLOSE",     1, FIXNUM(1)); }
/* >&n  */| GT AND NUMBER              { $$ = REDIRECT(p, "COPYWRITE", 2, FIXNUM(1), $3); }
/* >&n- */| GT AND NUMBER_MINUS        { const mrb_value vals[] = {
                                            REDIRECT(p, "COPYWRITE", 2, FIXNUM(1), $3),
                                            REDIRECT(p, "CLOSE",     1, $3)    };
                                         $$ = mrb_ary_new_from_values(p->state, 2, vals);
                                       }
/* n>&- */| NUMBER GT AND MINUS        { $$ = REDIRECT(p, "CLOSE",     1, $1); }
/* n>&n */| NUMBER GT AND NUMBER       { $$ = REDIRECT(p, "COPYWRITE", 2, $1, $4); }
/* n>&n-*/| NUMBER GT AND NUMBER_MINUS { const mrb_value vals[] = {
                                            REDIRECT(p, "COPYWRITE", 2, $1, $4),
                                            REDIRECT(p, "CLOSE",     1, $4)    };
                                         $$ = mrb_ary_new_from_values(p->state, 2, vals);
                                       }
/* &>   */| AND GT wordlist { const mrb_value vals[] = {
                                REDIRECT(p, "WRITE",     2, FIXNUM(1), $3),
                                REDIRECT(p, "COPYWRITE", 2, FIXNUM(2), FIXNUM(1)) };
                                $$ = mrb_ary_new_from_values(p->state, 2, vals);
                            }
/* >&   */| GT AND wordlist { const mrb_value vals[] = {
                                REDIRECT(p, "WRITE",     2, FIXNUM(1), $3),
                                REDIRECT(p, "COPYWRITE", 2, FIXNUM(2), FIXNUM(1)) };
                                $$ = mrb_ary_new_from_values(p->state, 2, vals);
                            }

/* >>   */| GT_GT wordlist        { $$ = REDIRECT(p, "APPEND", 2, FIXNUM(1), $2); }
/* n>>  */| NUMBER GT_GT wordlist { $$ = REDIRECT(p, "APPEND", 2, $1, $3); }

/* <>   */| LT GT wordlist        { $$ = REDIRECT(p, "READWRITE", 2, FIXNUM(0), $3); }
/* n<>  */| NUMBER LT GT wordlist { $$ = REDIRECT(p, "READWRITE", 2, $1, $4); }

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
    MRB_CONST_SET(mrb, tt, AND);
    MRB_CONST_SET(mrb, tt, AND_AND);
    MRB_CONST_SET(mrb, tt, GT);
    MRB_CONST_SET(mrb, tt, GT_GT);
    MRB_CONST_SET(mrb, tt, LT);
    MRB_CONST_SET(mrb, tt, MINUS);
    MRB_CONST_SET(mrb, tt, NUMBER);
    MRB_CONST_SET(mrb, tt, NUMBER_MINUS);
    MRB_CONST_SET(mrb, tt, OR);
    MRB_CONST_SET(mrb, tt, OR_OR);
    MRB_CONST_SET(mrb, tt, SEMICOLON);
    MRB_CONST_SET(mrb, tt, WORD);
}
