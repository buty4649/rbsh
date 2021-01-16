#include "mruby.h"

mrb_value mrb_reddish_parser_parse(mrb_state*, mrb_value);
mrb_value mrb_reddish_parser_debug(mrb_state*, mrb_value);

void mrb_mruby_reddish_parser_gem_init(mrb_state* mrb) {
    struct RClass* rp;

    rp = mrb_define_module(mrb, "ReddishParser");
    mrb_define_module_function(mrb, rp, "parse", mrb_reddish_parser_parse, MRB_ARGS_REQ(2));
    mrb_define_module_function(mrb, rp, "debug=", mrb_reddish_parser_debug, MRB_ARGS_REQ(1));
}

void mrb_mruby_reddish_parser_gem_final(mrb_state* mrb) {
}
