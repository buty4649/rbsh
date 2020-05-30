%{
#include "mruby.h"
#include "mruby/string.h"
#include "mruby/array.h"
#include "mruby/variable.h"

typedef struct cmdline_parse_state {
    mrb_state* state;
    mrb_value  parser_class;
    mrb_value  result;
    mrb_value  action_class;
} cmdline_parse_state;

#define ACTION(p, n, c, ...)   mrb_funcall(p->state, p->action_class, n, c, __VA_ARGS__)
#define MRB_CONST_SET(s, c, v) mrb_const_set( s, \
                                mrb_obj_value(c), \
                                mrb_intern_lit(s, #v), \
                                mrb_fixnum_value((mrb_int)v) \
                               )

#define YYDEBUG 1

static int yylex(void* , cmdline_parse_state*);
static void yyerror(cmdline_parse_state*, const char*);
static int yyparse(cmdline_parse_state*);
%}

%define api.value.type {mrb_value}
%define api.pure
%parse-param {cmdline_parse_state* p}
%lex-param {cmdline_parse_state* p}

%token WORD
%token AND AND_AND OR OR_OR
%token GT GT_GT LT
%type wordlist
%type simple_command simple_list
%start inputunit

%left AND_AND OR_OR

%%

inputunit : %empty
          | simple_list { p->result = $1; }

simple_list : simple_command                     { $$ = $1; }
            | simple_list AND_AND simple_command { $$ = ACTION(p, "make_and_command_connector", 2, $1, $3); }
            | simple_list OR_OR   simple_command { $$ = ACTION(p, "make_or_command_connector",  2, $1, $3); }

simple_command : wordlist { $$ = ACTION(p, "make_command", 1, $1); }
               | simple_command LT    wordlist { $$ = ACTION(p, "assign_read_redirect", 2, $1, $3); }
               | simple_command GT    wordlist { $$ = ACTION(p, "assign_write_redirect",2, $1, $3); }
               | simple_command GT_GT wordlist { $$ = ACTION(p, "assign_append_redirect", 2, $1, $3); }

wordlist : WORD          { $$ = ACTION(p, "make_word_list", 1, $1); }
         | wordlist WORD { $$ = ACTION(p, "add_to_word_list", 2, $1, $2); }

%%

int yylex(void* lval, cmdline_parse_state* p) {
    mrb_value token;
    int type;

    token = mrb_funcall(p->state, p->parser_class, "get_token", 0);

    *((YYSTYPE*)lval) = mrb_funcall(p->state, token, "word", 0);
    type = mrb_fixnum(mrb_funcall(p->state, token, "type", 0));

    return type;
}

void yyerror(cmdline_parse_state* p, const char* s){
    mrb_value str = mrb_str_new_cstr(p->state, s);
    mrb_funcall(p->state, p->parser_class, "error", 1, &str);
}

mrb_value mrb_reddish_parser_parse(mrb_state *mrb, mrb_value self) {
    mrb_value inputline;
    cmdline_parse_state pstate;
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
    MRB_CONST_SET(mrb, tt, AND);
    MRB_CONST_SET(mrb, tt, AND_AND);
    MRB_CONST_SET(mrb, tt, OR);
    MRB_CONST_SET(mrb, tt, OR_OR);
    MRB_CONST_SET(mrb, tt, GT);
    MRB_CONST_SET(mrb, tt, GT_GT);
    MRB_CONST_SET(mrb, tt, LT);
}
