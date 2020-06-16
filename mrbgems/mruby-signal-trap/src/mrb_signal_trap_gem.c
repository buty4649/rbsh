#include <signal.h>
#include <pthread.h>

#include "mruby.h"
#include "mruby/array.h"
#include "mruby/class.h"
#include "mruby/data.h"

typedef struct {
    int signal;
    void* data;
    pthread_t thread;
    mrb_bool alive;
} mrb_sigtrap_context;

void thread_cancel(mrb_sigtrap_context* ctxt) {
    sigset_t mask;
    if (ctxt->alive) {
        sigemptyset(&mask);
        sigaddset(&mask, ctxt->signal);
        pthread_sigmask(SIG_UNBLOCK, &mask, NULL);

        pthread_cancel(ctxt->thread);
        pthread_join(ctxt->thread, NULL);

        ctxt->alive = FALSE;
    }
}

static void sigtrap_free(mrb_state* mrb, void* p) {
    mrb_sigtrap_context* ctxt = (mrb_sigtrap_context*)p;
    if (ctxt->data) mrb_free(mrb, ctxt->data);
    thread_cancel(ctxt);
    mrb_free(mrb, p);
}
static const struct mrb_data_type mrb_sigtrap_context_type = {
    "mrb_sigtrap_context", sigtrap_free,
};

mrb_value mrb_initialize(mrb_state* mrb, mrb_value self) {
    mrb_sigtrap_context* ctxt;

    ctxt = (mrb_sigtrap_context*)mrb_malloc(mrb, sizeof(mrb_sigtrap_context));
    ctxt->signal = -1;
    ctxt->data = NULL;
    ctxt->thread = NULL;
    ctxt->alive = FALSE;
    mrb_data_init(self, ctxt, &mrb_sigtrap_context_type);

    return self;
}

mrb_value mrb_initialize_copy(mrb_state* mrb, mrb_value self) {
    // do not allow copy
    mrb_raise(mrb, E_NOTIMP_ERROR, "initialize_copy not implimented");
    return mrb_nil_value();
}

void sigint_handler(void* p) {
    mrb_sigtrap_context* ctxt = (mrb_sigtrap_context*)p;
    sigset_t mask;
    int sig;

    sigemptyset(&mask);
    sigaddset(&mask, SIGINT);

    for (;;) {
        sigwait(&mask, &sig);
        killpg(*((int*)ctxt->data), SIGINT);
    }
}

mrb_value mrb_start_sigint_trap(mrb_state* mrb, mrb_value self) {
    mrb_int pgid;
    mrb_sigtrap_context* ctxt = (mrb_sigtrap_context*)DATA_PTR(self);
    sigset_t mask;

    mrb_get_args(mrb, "i", &pgid);

    if (ctxt->alive) return self;
    if (ctxt->data == NULL) {
        ctxt->data = mrb_malloc(mrb, sizeof(int));
    }
    *((int*)ctxt->data) = pgid;
    ctxt->signal = SIGINT;

    sigemptyset(&mask);
    sigaddset(&mask, SIGINT);
    if (pthread_sigmask(SIG_BLOCK, &mask, NULL) != 0) {
        mrb_raise(mrb, E_RUNTIME_ERROR, "pthread_sigmask error");
    }

    if (pthread_create(&ctxt->thread, NULL, sigint_handler, ctxt) != 0) {
        mrb_raise(mrb, E_RUNTIME_ERROR, "pthread_create error");
    }
    ctxt->alive = TRUE;

    return self;
}

mrb_value mrb_stop_trap(mrb_state* mrb, mrb_value self) {
    mrb_sigtrap_context* ctxt = (mrb_sigtrap_context*)DATA_PTR(self);
    thread_cancel(ctxt);
}

void mrb_mruby_signal_trap_gem_init(mrb_state* mrb) {
    struct RClass* st;

    st = mrb_define_class(mrb, "SignalTrap", mrb->object_class);
    MRB_SET_INSTANCE_TT(st, MRB_TT_DATA);
    mrb_define_method(mrb, st, "initialize", mrb_initialize, MRB_ARGS_NONE());
    mrb_define_method(mrb, st, "initialize_copy", mrb_initialize_copy, MRB_ARGS_NONE());
    mrb_define_method(mrb, st, "start_sigint_trap", mrb_start_sigint_trap, MRB_ARGS_REQ(1));
    mrb_define_method(mrb, st, "stop_trap", mrb_stop_trap, MRB_ARGS_NONE());
}

void mrb_mruby_signal_trap_gem_final(mrb_state* mrb) {
}
