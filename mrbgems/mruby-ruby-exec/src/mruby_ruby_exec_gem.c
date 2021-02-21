#include <string.h>
#include "mruby.h"
#include "mruby/array.h"
#include "mruby/compile.h"
#include "mruby/error.h"
#include "mruby/string.h"
#include "mruby/variable.h"

mrb_value mrb_system_exit(mrb_state* mrb, mrb_value self) {
    mrb_int status;
    struct RClass* systemexit;
    mrb_value a[2], e;

    if (mrb_get_args(mrb, "|i", &status) == 0) {
        status = 0;
    }

    systemexit = mrb_class_get_under(mrb, mrb->kernel_module, "SystemExit");

    a[0] = mrb_fixnum_value(status);
    a[1] = mrb_str_new_cstr(mrb, "exit");
    e = mrb_obj_new(mrb, systemexit, 2, a);

    mrb_exc_raise(mrb, e);

    return mrb_nil_value();
}

mrb_value mrb_ruby_exec(mrb_state* mrb, mrb_value self) {
    char* code;
    char* filename;
    mrb_value* ary;
    mrb_int len;
    int ai, i;
    mrbc_context* c;
    mrb_value argv;
    mrb_int r = 0;
    mrb_state* m = mrb_open();

    ai = mrb_gc_arena_save(mrb);
    mrb_get_args(mrb, "zz|*", &code, &filename, &ary, &len);

    argv = mrb_ary_new_capa(m, len);
    for (i=0; i<len; i++) {
        mrb_ary_push(m, argv, mrb_str_new_cstr(m, mrb_str_to_cstr(mrb, ary[i])));
    }

    c = mrbc_context_new(m);
    mrbc_filename(m, c, filename);
    mrb_define_global_const(m, "ARGV", argv);
    mrb_gv_set(m, mrb_intern_lit(m, "$0"), mrb_str_new_cstr(m, filename));
    mrb_undef_method(m, m->kernel_module, "exit");
    mrb_define_module_function(m, m->kernel_module, "exit", mrb_system_exit, MRB_ARGS_OPT(1));

    mrb_load_string_cxt(m, code, c);

    if (m->exc) {
        mrb_value e = mrb_obj_value(m->exc);
        mrb_value c = mrb_funcall(m, mrb_funcall(m, e, "class", 0), "to_s", 0);
        const char* name = mrb_str_to_cstr(m, c);
        if (strcmp("SystemExit", name) == 0) {
            r = mrb_fixnum(mrb_funcall(m, e, "status", 0));
            m->exc = NULL;
        } else {
            mrb_print_error(m);
            r = 1;
        }
    }

    mrb_gc_arena_restore(mrb, ai);
    mrbc_context_free(m, c);

    mrb_close(m);
    m = NULL;

    return mrb_fixnum_value(r);
}

void mrb_mruby_ruby_exec_gem_init(mrb_state* mrb) {
    struct RClass* ruby;

    ruby = mrb_define_module(mrb, "Ruby");
    mrb_define_module_function(mrb, ruby, "exec", mrb_ruby_exec, MRB_ARGS_REQ(2)|MRB_ARGS_ANY());
}

void mrb_mruby_ruby_exec_gem_final(mrb_state* mrb) {
}
