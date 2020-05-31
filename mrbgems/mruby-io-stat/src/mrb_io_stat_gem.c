#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#include <errno.h>
#include "mruby.h"
#include "mruby/error.h"
#include "mruby/ext/io.h"

mrb_value mrb_io_stat(mrb_state* mrb, mrb_value self) {
    mrb_int fd;
    struct stat stat;
    int ret, ai;
    mrb_value mrb_stat;
    mrb_value ary[1];
    struct RClass* file_class = mrb_class_get(mrb, "File");

    fd = mrb_fixnum(mrb_io_fileno(mrb, self));
    ret = fstat(fd, &stat);

    if (ret == -1) {
        mrb_sys_fail(mrb, "stat");
    }

    ai = mrb_gc_arena_save(mrb);

    ary[0] = mrb_fixnum_value(stat.st_mode);

    mrb_stat = mrb_obj_new(mrb, mrb_class_get_under(mrb, file_class, "Stat"), 1, &ary);

    mrb_gc_arena_restore(mrb, ai);
    return mrb_stat;
}

void mrb_mruby_io_stat_gem_init(mrb_state* mrb) {
    struct RClass* io;
    struct RClass* file_class;
    struct RClass* struct_class;

    io = mrb_class_get(mrb, "IO");
    mrb_define_method(mrb, io, "stat", mrb_io_stat, MRB_ARGS_NONE());

}

void mrb_mruby_io_stat_gem_final(mrb_state* mrb) {
}
