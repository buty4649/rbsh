#include <unistd.h>
#include <fcntl.h>

#include "mruby.h"
#include "mruby/data.h"
#include "mruby/ext/io.h"

mrb_value mrb_io_fcntl(mrb_state* mrb, mrb_value self) {
    mrb_int cmd, arg, fd;

    arg = 0;
    mrb_get_args(mrb, "ii|i", &fd, &cmd, &arg);

    return mrb_fixnum_value(fcntl(fd, cmd, arg));
}


void mrb_mruby_io_fcntl_gem_init(mrb_state* mrb) {
    struct RClass* io;

    io = mrb_class_get(mrb, "IO");
    mrb_define_class_method(mrb, io, "fcntl", mrb_io_fcntl, MRB_ARGS_ARG(2, 1));
    mrb_define_const(mrb, io, "F_DUPFD", mrb_fixnum_value(F_DUPFD));
}

void mrb_mruby_io_fcntl_gem_final(mrb_state* mrb) {
}
