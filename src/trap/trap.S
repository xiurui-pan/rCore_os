
.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .global __alltraps
    .global __restore
    .align 2
__alltraps:

    csrrw sp, sscratch, sp
    # now sp->kernal stack, sscratch->user stack
    # allocate a TrapContext on kernal stack
    addi sp, sp, -34*8
    # save general-purpose registers 
    sd x1, 1*8(sp)
    # skip x2, do it later
    sd x3, 3*8(sp)
    # skip x4, application does not use it
    # save x4~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernal stack
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp 
    call trap_handler

__restore:
    # case1: start running app by __restore
    # case2: back to U after handling trap
    mv sp, a0
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # restore general-purpose register except sp/tp
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    ld x3, 3*8(sp)
    ld x1, 1*8(sp)
    # release TrapContext on kernal stack
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    sret