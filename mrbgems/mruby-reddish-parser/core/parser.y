%{
#include <string.h>
#include <stdio.h>
#include "mruby.h"
#include "mruby/string.h"
#include "mruby/array.h"
#include "mruby/variable.h"

typedef struct parser_state {
    mrb_state* state;
    mrb_value  result;
    mrb_value  action;
} parser_state;

#define ACTION(p, n, c, ...)   mrb_funcall(p->state, p->action, n, c, __VA_ARGS__)
#define COMMAND(p, e)          ACTION(p, "on_command", 1, e)
#define CONNECTOR(p, t, a, b)  ACTION(p, "on_connector", 3, mrb_symbol_value(mrb_intern_cstr(p->state, t)), a, b)
#define PIPELINE(p, a, b, r)   ACTION(p, "on_pipeline", 3, a, b, r)
#define REDIRECT(p, t, c, ...) ACTION(p, "on_redirect", (c+1), mrb_symbol_value(mrb_intern_cstr(p->state, t)), __VA_ARGS__)
#define SIMPLELIST(p, c, a)    ACTION(p, "on_simple_list", 2, c, a)
#define WORD(p, w)             ACTION(p, "on_word", 1, w)

#define FIXNUM(i) mrb_fixnum_value(i)
#define BOOL(b)   mrb_bool_value(b)

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
| simple_list { p->result = $1; }

simple_list
: connector { $$ = SIMPLELIST(p, $1, BOOL(0)); }
| connector AND       { $$ = SIMPLELIST(p, $1, BOOL(1)); }
| connector SEMICOLON { $$ = SIMPLELIST(p, $1, BOOL(0)); }

connector
: connector AND_AND   pipeline { $$ = CONNECTOR(p, "and", $1, $3); }
| connector OR_OR     pipeline { $$ = CONNECTOR(p, "or",  $1, $3); }
| connector AND       pipeline { $$ = CONNECTOR(p, "async", $1, $3); }
| connector SEMICOLON pipeline { $$ = CONNECTOR(p, "semicolon", $1, $3); }
| pipeline

pipeline
: pipeline OR command     { $$ = PIPELINE(p, $1, $3, BOOL(0)); }
| pipeline OR_AND command { $$ = PIPELINE(p, $1, $3, BOOL(1)); }
| command

command
: simple_command { $$ = COMMAND(p, $1); }

simple_command
: simple_command_element { $$ = mrb_ary_new_from_values(p->state, 1, &$1); }
| simple_command simple_command_element { mrb_ary_push(p->state, $1, $2); $$ = $1; }

simple_command_element
: WORD { $$ = WORD(p, $1); }
| redirect

redirect
/* <    */: LT WORD                    { $$ = REDIRECT(p, "read",     2, FIXNUM(0), $2); }
/* n<   */| NUMBER LT WORD             { $$ = REDIRECT(p, "read",     2, $1, $3); }
/* <&-  */| LT_AND MINUS               { $$ = REDIRECT(p, "close",    1, FIXNUM(0)); }
/* <&n  */| LT_AND NUMBER              { $$ = REDIRECT(p, "copyread", 2, FIXNUM(0), $2); }
/* <&n- */| LT_AND NUMBER MINUS        { $$ = REDIRECT(p, "copyreadclose", 2, FIXNUM(0), $2); }
/* n<&- */| NUMBER LT_AND MINUS        { $$ = REDIRECT(p, "close",    1, $1); }
/* n<&n */| NUMBER LT_AND NUMBER       { $$ = REDIRECT(p, "copyread", 2, $1, $3); }
/* n<&n-*/| NUMBER LT_AND NUMBER MINUS { $$ = REDIRECT(p, "copyreadclose", 2, $1, $3); }
/* >    */| GT WORD                    { $$ = REDIRECT(p, "write",     2, FIXNUM(1), $2); }
/* n>   */| NUMBER GT WORD             { $$ = REDIRECT(p, "write",     2, $1, $3); }
/* >&-  */| GT_AND MINUS               { $$ = REDIRECT(p, "close",     1, FIXNUM(1)); }
/* >&n  */| GT_AND NUMBER              { $$ = REDIRECT(p, "copywrite", 2, FIXNUM(1), $2); }
/* >&n- */| GT_AND NUMBER MINUS        { $$ = REDIRECT(p, "copywriteclose", 2, FIXNUM(1), $2); }
/* n>&- */| NUMBER GT_AND MINUS        { $$ = REDIRECT(p, "close",     1, $1); }
/* n>&n */| NUMBER GT_AND NUMBER       { $$ = REDIRECT(p, "copywrite", 2, $1, $3); }
/* n>&n-*/| NUMBER GT_AND NUMBER MINUS { $$ = REDIRECT(p, "copywriteclose", 2, $1, $3); }
/* &>   */| AND_GT WORD                { $$ = REDIRECT(p, "copystdoutstderr", 3, FIXNUM(1), FIXNUM(2), $2); }
/* >&   */| GT_AND WORD                { $$ = REDIRECT(p, "copystdoutstderr", 3, FIXNUM(1), FIXNUM(2), $2); }
/* >>   */| GT_GT WORD                 { $$ = REDIRECT(p, "append", 2, FIXNUM(1), $2); }
/* n>>  */| NUMBER GT_GT WORD          { $$ = REDIRECT(p, "append", 2, $1, $3); }
/* <>   */| LT_GT WORD                 { $$ = REDIRECT(p, "readwrite", 2, FIXNUM(0), $2); }
/* n<>  */| NUMBER LT_GT WORD          { $$ = REDIRECT(p, "readwrite", 2, $1, $3); }

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
    mrb_funcall(p->state, p->action, "on_error", 1, str);
}


mrb_value mrb_reddish_parser_parse(mrb_state *mrb, mrb_value self) {
    mrb_value line, action;
    struct RClass *action_class;
    parser_state pstate;

    mrb_get_args(mrb, "S", &line);

    action_class = mrb_class_get_under(mrb, mrb_module_get(mrb, "ReddishParser"), "Action");
    action = mrb_obj_new(mrb, action_class, 1, &line);

    pstate.state = mrb;
    pstate.result = mrb_nil_value();
    pstate.action = action;

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
