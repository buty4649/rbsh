#include <unistd.h>
#include "mruby.h"

mrb_value mrb_io_dup2(mrb_state* mrb, mrb_value self) {
    mrb_int   old_fd, new_fd;

    mrb_get_args(mrb, "ii", &old_fd, &new_fd);

    return mrb_fixnum_value(dup2(old_fd, new_fd));
}

void mrb_mruby_io_dup2_gem_init(mrb_state* mrb) {
    struct RClass* io;

    io = mrb_class_get(mrb, "IO");
    mrb_define_class_method(mrb, io, "dup2", mrb_io_dup2, MRB_ARGS_REQ(2));
}

void mrb_mruby_io_dup2_gem_final(mrb_state* mrb) {
}
