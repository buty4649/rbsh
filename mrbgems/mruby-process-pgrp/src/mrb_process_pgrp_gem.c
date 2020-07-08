#include <sys/types.h>
#include <unistd.h>

#include "mruby.h"
#include "mruby/error.h"

mrb_value mrb_p_getpgid(mrb_state *mrb, mrb_value self) {
    mrb_value pid;
    pid_t pgid;

    mrb_get_args(mrb, "i", &pid);
    pgid = getpgid((pid_t)mrb_fixnum(pid));

    if (pgid == -1) {
        mrb_sys_fail(mrb, "getpgid");
    }

    return mrb_fixnum_value((mrb_int)pgid);
}

mrb_value mrb_p_setpgid(mrb_state *mrb, mrb_value self) {
    mrb_value pid, pgid;

    mrb_get_args(mrb, "ii", &pid, &pgid);
    if (setpgid((pid_t)mrb_fixnum(pid), (pid_t)mrb_fixnum(pgid)) == -1 ) {
        mrb_sys_fail(mrb, "setpgid");
    }

    return mrb_fixnum_value(0);
}

void mrb_mruby_process_pgrp_gem_init(mrb_state* mrb) {
    struct RClass* p;

    p = mrb_module_get(mrb, "Process");
    mrb_define_class_method(mrb, p, "getpgid", mrb_p_getpgid, MRB_ARGS_REQ(1));
    mrb_define_class_method(mrb, p, "setpgid", mrb_p_setpgid, MRB_ARGS_REQ(2));
}

void mrb_mruby_process_pgrp_gem_final(mrb_state* mrb) {
}
