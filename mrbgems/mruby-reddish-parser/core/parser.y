%{
#include <string.h>
#include <stdio.h>
#include "mruby.h"
#include "mruby/string.h"
#include "mruby/array.h"
#include "mruby/variable.h"

typedef struct parser_state {
    mrb_state* state;
    mrb_value  lexer;
    mrb_value  result;
    mrb_value  action;
} parser_state;

#define ACTION(p, n, c, ...)   mrb_funcall(p->state, p->action, n, c, __VA_ARGS__)
#define REDIRECT(p, t, c, ...) ACTION(p, "on_redirect", (c+1), mrb_symbol_value(mrb_intern_cstr(p->state, t)), __VA_ARGS__)
#define CONNECTOR(p, t, c, ...)ACTION(p, "on_connector",(c+1), mrb_symbol_value(mrb_intern_cstr(p->state, t)), __VA_ARGS__)
#define FIXNUM(i) mrb_fixnum_value(i)
#define BOOL(b)   mrb_bool_value(b)
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
%token AND AND_AND OR OR_OR OR_AND SEMICOLON
%token GT GT_GT AND_GT GT_AND LT LT_AND LT_GT
%start inputunit

%%

inputunit
: %empty
| command_list { p->result = $1; }

command_list
: simple_command     { $$ = ACTION(p, "on_command_list", 1, $1); }
| simple_command AND { $$ = ACTION(p, "on_command_list", 2, $1, BOOL(1)); }
| simple_command SEMICOLON     { $$ = ACTION(p, "on_command_list", 1, $1); }
| simple_command AND SEMICOLON { $$ = ACTION(p, "on_command_list", 2, $1, BOOL(1)); }

simple_command
: pipeline
| simple_command AND_AND   pipeline { $$ = CONNECTOR(p, "and", 2, $1, $3); }
| simple_command OR_OR     pipeline { $$ = CONNECTOR(p, "or",  2, $1, $3); }
| simple_command AND       pipeline { $$ = CONNECTOR(p, "async", 2, $1, $3); }
| simple_command SEMICOLON pipeline { $$ = CONNECTOR(p, "semicolon", 2, $1, $3); }

pipeline
: command
| pipeline OR command     { $$ = ACTION(p, "on_pipeline", 2, $1, $3); }
| pipeline OR_AND command { mrb_value r = REDIRECT(p, "COPYWRITE", 2, FIXNUM(2), FIXNUM(1));
                                 ACTION(p, "on_command", 2, $3, r);
                            $$ = ACTION(p, "on_pipeline", 2, $1, $3); }
command
: wordlist { $$ = ACTION(p, "on_command", 1, $1); }
| wordlist redirect_list { $$ = ACTION(p, "on_command", 2, $1, $2); }

redirect_list
: redirect { $$ = ACTION(p, "on_redirect_list", 1, $1); }
| redirect_list redirect { $$ = ACTION(p, "on_redirect_list", 2, $2, $1); }

redirect
/* <    */: LT wordlist                { $$ = REDIRECT(p, "read",     2, FIXNUM(0), $2); }
/* n<   */| NUMBER LT wordlist         { $$ = REDIRECT(p, "read",     2, $1, $3); }
/* <&-  */| LT_AND MINUS               { $$ = REDIRECT(p, "close",    1, FIXNUM(0)); }
/* <&n  */| LT_AND NUMBER              { $$ = REDIRECT(p, "copyread", 2, FIXNUM(0), $2); }
/* <&n- */| LT_AND NUMBER MINUS        { const mrb_value vals[] = {
                                            REDIRECT(p, "copyread", 2, FIXNUM(0), $2),
                                            REDIRECT(p, "close",    1, $2) };
                                         $$ = mrb_ary_new_from_values(p->state, 2, vals);
                                       }
/* n<&- */| NUMBER LT_AND MINUS        { $$ = REDIRECT(p, "close",    1, $1); }
/* n<&n */| NUMBER LT_AND NUMBER       { $$ = REDIRECT(p, "copyread", 2, $1, $3); }
/* n<&n-*/| NUMBER LT_AND NUMBER MINUS { const mrb_value vals[] = {
                                            REDIRECT(p, "copyread", 2, $1, $3),
                                            REDIRECT(p, "close",    1, $3)    };
                                         $$ = mrb_ary_new_from_values(p->state, 2, vals);
                                       }

/* >    */| GT wordlist                { $$ = REDIRECT(p, "write",     2, FIXNUM(1), $2); }
/* n>   */| NUMBER GT wordlist         { $$ = REDIRECT(p, "write",     2, $1, $3); }
/* >&-  */| GT_AND MINUS               { $$ = REDIRECT(p, "close",     1, FIXNUM(1)); }
/* >&n  */| GT_AND NUMBER              { $$ = REDIRECT(p, "copywrite", 2, FIXNUM(1), $2); }
/* >&n- */| GT_AND NUMBER MINUS        { const mrb_value vals[] = {
                                            REDIRECT(p, "copywrite", 2, FIXNUM(1), $2),
                                            REDIRECT(p, "close",     1, $2)    };
                                         $$ = mrb_ary_new_from_values(p->state, 2, vals);
                                       }
/* n>&- */| NUMBER GT_AND MINUS        { $$ = REDIRECT(p, "close",     1, $1); }
/* n>&n */| NUMBER GT_AND NUMBER       { $$ = REDIRECT(p, "copywrite", 2, $1, $3); }
/* n>&n-*/| NUMBER GT_AND NUMBER MINUS { const mrb_value vals[] = {
                                            REDIRECT(p, "copywrite", 2, $1, $3),
                                            REDIRECT(p, "close",     1, $3)    };
                                         $$ = mrb_ary_new_from_values(p->state, 2, vals);
                                       }
/* &>   */| AND_GT wordlist { const mrb_value vals[] = {
                                REDIRECT(p, "write",     2, FIXNUM(1), $2),
                                REDIRECT(p, "copywrite", 2, FIXNUM(2), FIXNUM(1)) };
                                $$ = mrb_ary_new_from_values(p->state, 2, vals);
                            }
/* >&   */| GT_AND wordlist { const mrb_value vals[] = {
                                REDIRECT(p, "write",     2, FIXNUM(1), $2),
                                REDIRECT(p, "copywrite", 2, FIXNUM(2), FIXNUM(1)) };
                                $$ = mrb_ary_new_from_values(p->state, 2, vals);
                            }

/* >>   */| GT_GT wordlist        { $$ = REDIRECT(p, "append", 2, FIXNUM(1), $2); }
/* n>>  */| NUMBER GT_GT wordlist { $$ = REDIRECT(p, "append", 2, $1, $3); }

/* <>   */| LT_GT wordlist        { $$ = REDIRECT(p, "readwrite", 2, FIXNUM(0), $2); }
/* n<>  */| NUMBER LT_GT wordlist { $$ = REDIRECT(p, "readwrite", 2, $1, $3); }

wordlist
: WORD          { $$ = ACTION(p, "on_word", 1, $1); }
| wordlist WORD { $$ = ACTION(p, "on_word", 2, $2, $1); }

%%
static const struct token_type {
    const char *name;
    int type;
} token_types[] = {
    {"&",  AND},
    {"&&", AND_AND},
    {"&>", AND_GT},
    {">",  GT},
    {">&", GT_AND},
    {">>", GT_GT},
    {"<",  LT},
    {"<&", LT_AND},
    {"<>", LT_GT},
    {"-",  MINUS},
    {"|",  OR},
    {"|&", OR_AND},
    {"||", OR_OR},
    {";",  SEMICOLON},
    {"eof", YYEOF},
    {"number", NUMBER},
    {"word", WORD},
    {NULL, 0}
};

int sym2tt(const char *sym) {
    const struct token_type *tt;
    for (tt = token_types; tt->name; tt++) {
        if (strcmp(tt->name, sym) == 0) {
            return tt->type;
        }
    }
    return 0;
}

int yylex(void* lval, parser_state* p) {
    mrb_value lexer, token;
    int type;

    lexer = mrb_iv_get(p->state, p->action, mrb_intern_lit(p->state, "@__lexer"));
    token = mrb_funcall(p->state, lexer, "get_token", 0);

    if (mrb_nil_p(token)) {
        return YYEOF;
    }

    type = sym2tt(mrb_sym2name(p->state, mrb_symbol(mrb_ary_ref(p->state, token, 0))));
    *((YYSTYPE*)lval) = mrb_ary_ref(p->state, token, 1);

    return type;
}

void yyerror(parser_state* p, const char* s){
    mrb_value str = mrb_str_new_cstr(p->state, s);
    mrb_funcall(p->state, p->lexer, "error", 1, str);
}

mrb_value mrb_reddish_parser_parse(mrb_state *mrb, mrb_value self) {
    parser_state pstate;

    pstate.state = mrb;
    pstate.result = mrb_nil_value();
    pstate.action = self;

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
