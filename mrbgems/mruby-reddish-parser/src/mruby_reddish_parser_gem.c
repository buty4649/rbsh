#include "mruby.h"

mrb_value mrb_reddish_parser_parse(mrb_state*, mrb_value);
mrb_value mrb_reddish_parser_debug(mrb_state*, mrb_value);

void mrb_mruby_reddish_parser_gem_init(mrb_state* mrb) {
    struct RClass* rp;
    struct RClass* action;

    rp = mrb_define_module(mrb, "ReddishParser");
    action = mrb_define_class_under(mrb, rp, "Action", mrb->object_class);
    mrb_define_method(mrb, action, "parse", mrb_reddish_parser_parse, MRB_ARGS_NONE());
    mrb_define_module_function(mrb, rp, "debug=", mrb_reddish_parser_debug, MRB_ARGS_REQ(1));
}

void mrb_mruby_reddish_parser_gem_final(mrb_state* mrb) {
}
