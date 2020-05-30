#include "mruby.h"

mrb_value mrb_reddish_parser_parse(mrb_state*, mrb_value);
mrb_value mrb_reddish_parser_debug(mrb_state*, mrb_value);
void mrb_tokentype_initialize(mrb_state*, struct RClass*);

void mrb_mruby_reddish_parser_gem_init(mrb_state* mrb) {
    struct RClass* rp;
    struct RClass* tt;

    rp = mrb_define_module(mrb, "ReddishParser");
    mrb_define_module_function(mrb, rp, "parse", mrb_reddish_parser_parse, MRB_ARGS_REQ(1));
    mrb_define_module_function(mrb, rp, "debug=", mrb_reddish_parser_debug, MRB_ARGS_REQ(1));

    tt = mrb_define_class_under(mrb, rp, "TokenType", mrb->object_class);
    mrb_tokentype_initialize(mrb, tt);
}

void mrb_mruby_reddish_parser_gem_final(mrb_state* mrb) {
}
