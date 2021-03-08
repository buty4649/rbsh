#include <errno.h>
#include <signal.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>

#include <mruby.h>
#include <mruby/array.h>
#include <mruby/error.h>

volatile sig_atomic_t interrupt_state = 0;

void signal_handler(int sig, siginfo_t* info, void* ctx) {
    switch(sig) {
        case SIGINT:
            interrupt_state = 1;
            break;
    }
}

void mask_tty_signals(int how) {
    sigset_t sig;
    sigemptyset(&sig);
    sigaddset(&sig, SIGTTIN);
    sigaddset(&sig, SIGTTOU);
    sigaddset(&sig, SIGTSTP);
    sigprocmask(how, &sig, NULL);
}

mrb_value mrb_start_signal_handlers(mrb_state* mrb, mrb_value self) {
    struct sigaction sa;

    sa.sa_flags = SA_SIGINFO;
    sa.sa_handler = SIG_DFL;
    sa.sa_sigaction = signal_handler;
    sigemptyset(&sa.sa_mask);

    if (sigaction(SIGINT, &sa, NULL) == -1) {
        mrb_sys_fail(mrb, "sigaction");
    }

    return mrb_nil_value();
}

mrb_value mrb_reset_signal_handlers(mrb_state* mrb, mrb_value self) {
    struct sigaction sa;

    sa.sa_flags = SA_RESETHAND;
    sa.sa_handler = SIG_DFL;
    sa.sa_sigaction = NULL;
    sigemptyset(&sa.sa_mask);
    sigaction(SIGINT, &sa, NULL);

    return mrb_nil_value();
}

mrb_value mrb_wait_pgid(mrb_state* mrb, mrb_value self) {
    mrb_int pgid;
    mrb_value result;
    siginfo_t info;
    struct RClass* st;
    mrb_value o[2], ps;
    int exit_status;

    mrb_get_args(mrb, "i", &pgid);

    st = mrb_class_get_under(mrb, mrb_module_get(mrb, "Process"), "Status");
    result = mrb_ary_new(mrb);
    interrupt_state = 0;

    for (;;) {
        if (waitid(P_PGID, (pid_t)pgid, &info, WEXITED) == -1) {
            if (errno == EINTR) {
                // interuput from SIGINT
                if (interrupt_state) {
                    killpg((pid_t)pgid, SIGINT);
                }
                continue;
            }
            break;
        }

        exit_status = info.si_status;
        if (info.si_code != CLD_EXITED) {
            exit_status += 128;
            if (info.si_status == SIGINT) {
                interrupt_state = 1;
            }
        }
        o[0] = mrb_fixnum_value(info.si_pid);
        o[1] = mrb_fixnum_value(exit_status);
        ps = mrb_obj_new(mrb, st, 2, o);
        mrb_ary_push(mrb, result, ps);
    }

    return result;
}

mrb_value mrb_interrupt(mrb_state* mrb, mrb_value self) {
    return interrupt_state == 0 ? mrb_false_value() : mrb_true_value();
}

mrb_value mrb_reset_interrupt_state(mrb_state* mrb, mrb_value self) {
    interrupt_state = 0;
    return mrb_nil_value();
}

mrb_value mrb_tcgetpgrp(mrb_state* mrb, mrb_value self) {
    mrb_int fd;
    pid_t pgrp;

    mrb_get_args(mrb, "i", &fd);
    pgrp = tcgetpgrp(fd);

    if (pgrp == -1) {
        mrb_sys_fail(mrb, "tcgetpgrp");
    }
    return mrb_fixnum_value(pgrp);
}

mrb_value mrb_tcsetpgrp(mrb_state* mrb, mrb_value self) {
    mrb_int fd;
    pid_t pgrp;

    mrb_get_args(mrb, "ii", &fd, &pgrp);
    if (tcsetpgrp(fd, pgrp) == -1) {
        mrb_sys_fail(mrb, "tcsetpgrp");
    }
    return mrb_fixnum_value(0);
}

mrb_value mrb_ignore_tty_signals(mrb_state* mrb, mrb_value self) {
    mask_tty_signals(SIG_BLOCK);
    return mrb_nil_value();
}

mrb_value mrb_restore_tty_signals(mrb_state* mrb, mrb_value self) {
    mask_tty_signals(SIG_UNBLOCK);
    return mrb_nil_value();
}

void mrb_mruby_signal_handler_gem_init(mrb_state* mrb) {
    struct RClass* sh;

    sh = mrb_define_module(mrb, "SignalHandler");

    mrb_define_module_function(mrb, sh, "start_signal_handlers", mrb_start_signal_handlers, MRB_ARGS_REQ(1));
    mrb_define_module_function(mrb, sh, "reset_signal_handlers", mrb_reset_signal_handlers, MRB_ARGS_NONE());
    mrb_define_module_function(mrb, sh, "wait_pgid", mrb_wait_pgid, MRB_ARGS_REQ(1));
    mrb_define_module_function(mrb, sh, "interrupt?", mrb_interrupt, MRB_ARGS_NONE());
    mrb_define_module_function(mrb, sh, "reset_interrupt_state", mrb_reset_interrupt_state, MRB_ARGS_NONE());
    mrb_define_module_function(mrb, sh, "tcgetpgrp", mrb_tcgetpgrp, MRB_ARGS_REQ(1));
    mrb_define_module_function(mrb, sh, "tcsetpgrp", mrb_tcsetpgrp, MRB_ARGS_REQ(2));
    mrb_define_module_function(mrb, sh, "ignore_tty_signals", mrb_ignore_tty_signals, MRB_ARGS_NONE());
    mrb_define_module_function(mrb, sh, "restore_tty_signals", mrb_restore_tty_signals, MRB_ARGS_NONE());
}

void mrb_mruby_signal_handler_gem_final(mrb_state* mrb) {
}
