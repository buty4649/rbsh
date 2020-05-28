%{
#include <stdio.h>
#include <string.h>
#include <mruby.h>
#include <mruby/string.h>
#include <mruby/array.h>
#include <mruby/variable.h>

#define MRB_REDDISH_MODULE(s) mrb_module_get(s, "Reddish")
#define MRB_CLASS_GET(s, c)   mrb_class_get_under(s, MRB_REDDISH_MODULE(s), c)
#define MRB_MODULE_GET(s, c)  mrb_module_get_under(s, MRB_REDDISH_MODULE(s), c)
#define MRB_CLASS_GET2(s, c1, c2)  mrb_class_get_under(s, MRB_MODULE_GET(s, c1), c2)

#define MRB_CONST_SET(s, c, v) mrb_const_set( s, \
                                mrb_obj_value(c), \
                                mrb_intern_lit(s, #v), \
                                mrb_fixnum_value((mrb_int)v) \
                               )

#define MRB_COMMAND_CLASS(s)   MRB_CLASS_GET(s, "Command")
#define MRB_PARSER_CLASS(s)    MRB_CLASS_GET(s, "Parser")
#define MRB_WORD_LIST_CLASS(s) MRB_CLASS_GET(s, "WordList")


typedef struct cmdline_parse_state {
    mrb_state* state;
    mrb_value  parser_class;
    mrb_value  result;
} cmdline_parse_state;

#define YYDEBUG 1

static int yylex(void* , cmdline_parse_state*);
static void yyerror(cmdline_parse_state*, const char*);
static int yyparse(cmdline_parse_state*);

#define mrb_command_new(p, v)             mrb_obj_new(p->state, MRB_COMMAND_CLASS(p->state), 1, &v)

#define mrb_word_list_new(p, v)    mrb_obj_new(p->state, MRB_WORD_LIST_CLASS(p->state), 1, &v)
#define mrb_word_list_add(p, d, v) mrb_funcall(p->state, d, "add", 1, v)

static mrb_value mrb_command_connector_new(cmdline_parse_state*, mrb_value, mrb_value, const char*);
static mrb_value mrb_command_add_redirect(cmdline_parse_state*, mrb_value, mrb_value, const char*);

%}

%define api.pure
%parse-param {cmdline_parse_state* p}
%lex-param {cmdline_parse_state* p}

%union {
    mrb_value word;
    mrb_value wordlist;
    mrb_value command;
}

%token <word> WORD
%token AND_AND OR_OR GREATER_GREATER
%type <wordlist> wordlist
%type <command> simple_command simple_list
%start inputunit

%left AND_AND

%%

inputunit : %empty
          | simple_list { p->result = $1; }
simple_list : simple_command { $$ = $1; }
            | simple_list AND_AND simple_command { $$ = mrb_command_connector_new(p, $1, $3, "And"); }
            | simple_list OR_OR simple_command { $$ = mrb_command_connector_new(p, $1, $3, "Or"); }
simple_command : wordlist { $$ = mrb_command_new(p, $1); }
               | simple_command '<' wordlist { $$ = mrb_command_add_redirect(p, $1, $3, "Read"); }
               | simple_command '>' wordlist { $$ = mrb_command_add_redirect(p, $1, $3, "Write"); }
               | simple_command GREATER_GREATER wordlist { $$ = mrb_command_add_redirect(p, $1, $3, "Append"); }
wordlist : WORD { $$ = mrb_word_list_new(p, $1); }
         | wordlist WORD { $$ = mrb_word_list_add(p, $1, $2); }

%%

int yylex(void* lval, cmdline_parse_state* p) {
    mrb_value token;
    int type;

    token = mrb_funcall(p->state, p->parser_class, "get_token", 0);

    ((YYSTYPE*)lval)->word = mrb_funcall(p->state, token, "word", 0);
    type = mrb_fixnum(mrb_funcall(p->state, token, "type", 0));

    return type;
}

void yyerror(cmdline_parse_state* p, const char* s){
    mrb_value str = mrb_str_new_cstr(p->state, s);
    mrb_funcall(p->state, p->parser_class, "error", 1, &str);
}

mrb_value commandline_f_parse(mrb_state *mrb, mrb_value self) {
    mrb_value inputline;
    cmdline_parse_state pstate;

    mrb_get_args(mrb, "S", &inputline);

    pstate.state = mrb;
    pstate.parser_class = mrb_obj_new(mrb, MRB_PARSER_CLASS(mrb), 1, &inputline);
    pstate.result = mrb_nil_value();

    yyparse(&pstate);

    if (mrb_nil_p(pstate.result)) {
        return mrb_nil_value();
    }

    return pstate.result;
}

mrb_value parser_f_debug(mrb_state *mrb, mrb_value self) {
    mrb_int flag;
    mrb_get_args(mrb, "i", &flag);
    yydebug = flag;
    return mrb_fixnum_value(flag);
}

static mrb_value mrb_command_connector_new(cmdline_parse_state* p, mrb_value cmd1, mrb_value cmd2, const char* type) {
    struct RClass* c;
    mrb_value ary[] = {cmd1, cmd2};

    c = MRB_CLASS_GET2(p->state, "CommandConnector", type);
    return mrb_obj_new(p->state, c, 2, ary);
}

static mrb_value mrb_command_add_redirect(cmdline_parse_state* p, mrb_value cmd, mrb_value word, const char* type) {
    struct RClass* c;
    mrb_value ary[] = {word};
    mrb_value redirect;

    c = MRB_CLASS_GET2(p->state, "Redirect", type);
    redirect = mrb_obj_new(p->state, c, 1, ary);

    return mrb_funcall(p->state, cmd, "redirect_add", 1, redirect);
}

void mrb_reddish_gem_init(mrb_state* mrb) {
    struct RClass* reddish;
    struct RClass* commandline;
    struct RClass* parser;
    struct RClass* tt;

    reddish = mrb_define_module(mrb, "Reddish");
    commandline = mrb_define_class_under(mrb, reddish, "Commandline", mrb->object_class);
    mrb_define_class_method(mrb, commandline, "parse", commandline_f_parse, MRB_ARGS_REQ(1));

    parser = mrb_define_class_under(mrb, reddish, "Parser", mrb->object_class);
    mrb_define_class_method(mrb, parser, "debug=", parser_f_debug, MRB_ARGS_REQ(1));

    tt = mrb_define_class_under(mrb, reddish, "TokenType", mrb->object_class);
    MRB_CONST_SET(mrb, tt, YYEOF);
    MRB_CONST_SET(mrb, tt, WORD);
    MRB_CONST_SET(mrb, tt, AND_AND);
    MRB_CONST_SET(mrb, tt, OR_OR);
    MRB_CONST_SET(mrb, tt, GREATER_GREATER);
}

void mrb_reddish_gem_final(mrb_state* mrb) {
}
