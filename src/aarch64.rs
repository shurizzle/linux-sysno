//! Syscalls for arch `aarch64`.

#![allow(non_camel_case_types)]

// This file is automatically generated. Do not edit.

/// List of system call numbers.
#[repr(usize)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sysno {
    /// See doc at [io_setup(2)](https://man7.org/linux/man-pages/man2/io_setup.2.html).
    io_setup = 0,
    /// See doc at [io_destroy(2)](https://man7.org/linux/man-pages/man2/io_destroy.2.html).
    io_destroy = 1,
    /// See doc at [io_submit(2)](https://man7.org/linux/man-pages/man2/io_submit.2.html).
    io_submit = 2,
    /// See doc at [io_cancel(2)](https://man7.org/linux/man-pages/man2/io_cancel.2.html).
    io_cancel = 3,
    /// See doc at [io_getevents(2)](https://man7.org/linux/man-pages/man2/io_getevents.2.html).
    io_getevents = 4,
    /// See doc at [setxattr(2)](https://man7.org/linux/man-pages/man2/setxattr.2.html).
    setxattr = 5,
    /// See doc at [lsetxattr(2)](https://man7.org/linux/man-pages/man2/lsetxattr.2.html).
    lsetxattr = 6,
    /// See doc at [fsetxattr(2)](https://man7.org/linux/man-pages/man2/fsetxattr.2.html).
    fsetxattr = 7,
    /// See doc at [getxattr(2)](https://man7.org/linux/man-pages/man2/getxattr.2.html).
    getxattr = 8,
    /// See doc at [lgetxattr(2)](https://man7.org/linux/man-pages/man2/lgetxattr.2.html).
    lgetxattr = 9,
    /// See doc at [fgetxattr(2)](https://man7.org/linux/man-pages/man2/fgetxattr.2.html).
    fgetxattr = 10,
    /// See doc at [listxattr(2)](https://man7.org/linux/man-pages/man2/listxattr.2.html).
    listxattr = 11,
    /// See doc at [llistxattr(2)](https://man7.org/linux/man-pages/man2/llistxattr.2.html).
    llistxattr = 12,
    /// See doc at [flistxattr(2)](https://man7.org/linux/man-pages/man2/flistxattr.2.html).
    flistxattr = 13,
    /// See doc at [removexattr(2)](https://man7.org/linux/man-pages/man2/removexattr.2.html).
    removexattr = 14,
    /// See doc at [lremovexattr(2)](https://man7.org/linux/man-pages/man2/lremovexattr.2.html).
    lremovexattr = 15,
    /// See doc at [fremovexattr(2)](https://man7.org/linux/man-pages/man2/fremovexattr.2.html).
    fremovexattr = 16,
    /// See doc at [getcwd(2)](https://man7.org/linux/man-pages/man2/getcwd.2.html).
    getcwd = 17,
    /// See doc at [lookup_dcookie(2)](https://man7.org/linux/man-pages/man2/lookup_dcookie.2.html).
    lookup_dcookie = 18,
    /// See doc at [eventfd2(2)](https://man7.org/linux/man-pages/man2/eventfd2.2.html).
    eventfd2 = 19,
    /// See doc at [epoll_create1(2)](https://man7.org/linux/man-pages/man2/epoll_create1.2.html).
    epoll_create1 = 20,
    /// See doc at [epoll_ctl(2)](https://man7.org/linux/man-pages/man2/epoll_ctl.2.html).
    epoll_ctl = 21,
    /// See doc at [epoll_pwait(2)](https://man7.org/linux/man-pages/man2/epoll_pwait.2.html).
    epoll_pwait = 22,
    /// See doc at [dup(2)](https://man7.org/linux/man-pages/man2/dup.2.html).
    dup = 23,
    /// See doc at [dup3(2)](https://man7.org/linux/man-pages/man2/dup3.2.html).
    dup3 = 24,
    /// See doc at [fcntl(2)](https://man7.org/linux/man-pages/man2/fcntl.2.html).
    fcntl = 25,
    /// See doc at [inotify_init1(2)](https://man7.org/linux/man-pages/man2/inotify_init1.2.html).
    inotify_init1 = 26,
    /// See doc at [inotify_add_watch(2)](https://man7.org/linux/man-pages/man2/inotify_add_watch.2.html).
    inotify_add_watch = 27,
    /// See doc at [inotify_rm_watch(2)](https://man7.org/linux/man-pages/man2/inotify_rm_watch.2.html).
    inotify_rm_watch = 28,
    /// See doc at [ioctl(2)](https://man7.org/linux/man-pages/man2/ioctl.2.html).
    ioctl = 29,
    /// See doc at [ioprio_set(2)](https://man7.org/linux/man-pages/man2/ioprio_set.2.html).
    ioprio_set = 30,
    /// See doc at [ioprio_get(2)](https://man7.org/linux/man-pages/man2/ioprio_get.2.html).
    ioprio_get = 31,
    /// See doc at [flock(2)](https://man7.org/linux/man-pages/man2/flock.2.html).
    flock = 32,
    /// See doc at [mknodat(2)](https://man7.org/linux/man-pages/man2/mknodat.2.html).
    mknodat = 33,
    /// See doc at [mkdirat(2)](https://man7.org/linux/man-pages/man2/mkdirat.2.html).
    mkdirat = 34,
    /// See doc at [unlinkat(2)](https://man7.org/linux/man-pages/man2/unlinkat.2.html).
    unlinkat = 35,
    /// See doc at [symlinkat(2)](https://man7.org/linux/man-pages/man2/symlinkat.2.html).
    symlinkat = 36,
    /// See doc at [linkat(2)](https://man7.org/linux/man-pages/man2/linkat.2.html).
    linkat = 37,
    /// See doc at [renameat(2)](https://man7.org/linux/man-pages/man2/renameat.2.html).
    renameat = 38,
    /// See doc at [umount2(2)](https://man7.org/linux/man-pages/man2/umount2.2.html).
    umount2 = 39,
    /// See doc at [mount(2)](https://man7.org/linux/man-pages/man2/mount.2.html).
    mount = 40,
    /// See doc at [pivot_root(2)](https://man7.org/linux/man-pages/man2/pivot_root.2.html).
    pivot_root = 41,
    /// See doc at [nfsservctl(2)](https://man7.org/linux/man-pages/man2/nfsservctl.2.html).
    nfsservctl = 42,
    /// See doc at [statfs(2)](https://man7.org/linux/man-pages/man2/statfs.2.html).
    statfs = 43,
    /// See doc at [fstatfs(2)](https://man7.org/linux/man-pages/man2/fstatfs.2.html).
    fstatfs = 44,
    /// See doc at [truncate(2)](https://man7.org/linux/man-pages/man2/truncate.2.html).
    truncate = 45,
    /// See doc at [ftruncate(2)](https://man7.org/linux/man-pages/man2/ftruncate.2.html).
    ftruncate = 46,
    /// See doc at [fallocate(2)](https://man7.org/linux/man-pages/man2/fallocate.2.html).
    fallocate = 47,
    /// See doc at [faccessat(2)](https://man7.org/linux/man-pages/man2/faccessat.2.html).
    faccessat = 48,
    /// See doc at [chdir(2)](https://man7.org/linux/man-pages/man2/chdir.2.html).
    chdir = 49,
    /// See doc at [fchdir(2)](https://man7.org/linux/man-pages/man2/fchdir.2.html).
    fchdir = 50,
    /// See doc at [chroot(2)](https://man7.org/linux/man-pages/man2/chroot.2.html).
    chroot = 51,
    /// See doc at [fchmod(2)](https://man7.org/linux/man-pages/man2/fchmod.2.html).
    fchmod = 52,
    /// See doc at [fchmodat(2)](https://man7.org/linux/man-pages/man2/fchmodat.2.html).
    fchmodat = 53,
    /// See doc at [fchownat(2)](https://man7.org/linux/man-pages/man2/fchownat.2.html).
    fchownat = 54,
    /// See doc at [fchown(2)](https://man7.org/linux/man-pages/man2/fchown.2.html).
    fchown = 55,
    /// See doc at [openat(2)](https://man7.org/linux/man-pages/man2/openat.2.html).
    openat = 56,
    /// See doc at [close(2)](https://man7.org/linux/man-pages/man2/close.2.html).
    close = 57,
    /// See doc at [vhangup(2)](https://man7.org/linux/man-pages/man2/vhangup.2.html).
    vhangup = 58,
    /// See doc at [pipe2(2)](https://man7.org/linux/man-pages/man2/pipe2.2.html).
    pipe2 = 59,
    /// See doc at [quotactl(2)](https://man7.org/linux/man-pages/man2/quotactl.2.html).
    quotactl = 60,
    /// See doc at [getdents64(2)](https://man7.org/linux/man-pages/man2/getdents64.2.html).
    getdents64 = 61,
    /// See doc at [lseek(2)](https://man7.org/linux/man-pages/man2/lseek.2.html).
    lseek = 62,
    /// See doc at [read(2)](https://man7.org/linux/man-pages/man2/read.2.html).
    read = 63,
    /// See doc at [write(2)](https://man7.org/linux/man-pages/man2/write.2.html).
    write = 64,
    /// See doc at [readv(2)](https://man7.org/linux/man-pages/man2/readv.2.html).
    readv = 65,
    /// See doc at [writev(2)](https://man7.org/linux/man-pages/man2/writev.2.html).
    writev = 66,
    /// See doc at [pread64(2)](https://man7.org/linux/man-pages/man2/pread64.2.html).
    pread64 = 67,
    /// See doc at [pwrite64(2)](https://man7.org/linux/man-pages/man2/pwrite64.2.html).
    pwrite64 = 68,
    /// See doc at [preadv(2)](https://man7.org/linux/man-pages/man2/preadv.2.html).
    preadv = 69,
    /// See doc at [pwritev(2)](https://man7.org/linux/man-pages/man2/pwritev.2.html).
    pwritev = 70,
    /// See doc at [sendfile(2)](https://man7.org/linux/man-pages/man2/sendfile.2.html).
    sendfile = 71,
    /// See doc at [pselect6(2)](https://man7.org/linux/man-pages/man2/pselect6.2.html).
    pselect6 = 72,
    /// See doc at [ppoll(2)](https://man7.org/linux/man-pages/man2/ppoll.2.html).
    ppoll = 73,
    /// See doc at [signalfd4(2)](https://man7.org/linux/man-pages/man2/signalfd4.2.html).
    signalfd4 = 74,
    /// See doc at [vmsplice(2)](https://man7.org/linux/man-pages/man2/vmsplice.2.html).
    vmsplice = 75,
    /// See doc at [splice(2)](https://man7.org/linux/man-pages/man2/splice.2.html).
    splice = 76,
    /// See doc at [tee(2)](https://man7.org/linux/man-pages/man2/tee.2.html).
    tee = 77,
    /// See doc at [readlinkat(2)](https://man7.org/linux/man-pages/man2/readlinkat.2.html).
    readlinkat = 78,
    /// See doc at [fstatat(2)](https://man7.org/linux/man-pages/man2/fstatat.2.html).
    fstatat = 79,
    /// See doc at [fstat(2)](https://man7.org/linux/man-pages/man2/fstat.2.html).
    fstat = 80,
    /// See doc at [sync(2)](https://man7.org/linux/man-pages/man2/sync.2.html).
    sync = 81,
    /// See doc at [fsync(2)](https://man7.org/linux/man-pages/man2/fsync.2.html).
    fsync = 82,
    /// See doc at [fdatasync(2)](https://man7.org/linux/man-pages/man2/fdatasync.2.html).
    fdatasync = 83,
    /// See doc at [sync_file_range2(2)](https://man7.org/linux/man-pages/man2/sync_file_range2.2.html).
    sync_file_range2 = 84,
    /// See doc at [timerfd_create(2)](https://man7.org/linux/man-pages/man2/timerfd_create.2.html).
    timerfd_create = 85,
    /// See doc at [timerfd_settime(2)](https://man7.org/linux/man-pages/man2/timerfd_settime.2.html).
    timerfd_settime = 86,
    /// See doc at [timerfd_gettime(2)](https://man7.org/linux/man-pages/man2/timerfd_gettime.2.html).
    timerfd_gettime = 87,
    /// See doc at [utimensat(2)](https://man7.org/linux/man-pages/man2/utimensat.2.html).
    utimensat = 88,
    /// See doc at [acct(2)](https://man7.org/linux/man-pages/man2/acct.2.html).
    acct = 89,
    /// See doc at [capget(2)](https://man7.org/linux/man-pages/man2/capget.2.html).
    capget = 90,
    /// See doc at [capset(2)](https://man7.org/linux/man-pages/man2/capset.2.html).
    capset = 91,
    /// See doc at [personality(2)](https://man7.org/linux/man-pages/man2/personality.2.html).
    personality = 92,
    /// See doc at [exit(2)](https://man7.org/linux/man-pages/man2/exit.2.html).
    exit = 93,
    /// See doc at [exit_group(2)](https://man7.org/linux/man-pages/man2/exit_group.2.html).
    exit_group = 94,
    /// See doc at [waitid(2)](https://man7.org/linux/man-pages/man2/waitid.2.html).
    waitid = 95,
    /// See doc at [set_tid_address(2)](https://man7.org/linux/man-pages/man2/set_tid_address.2.html).
    set_tid_address = 96,
    /// See doc at [unshare(2)](https://man7.org/linux/man-pages/man2/unshare.2.html).
    unshare = 97,
    /// See doc at [futex(2)](https://man7.org/linux/man-pages/man2/futex.2.html).
    futex = 98,
    /// See doc at [set_robust_list(2)](https://man7.org/linux/man-pages/man2/set_robust_list.2.html).
    set_robust_list = 99,
    /// See doc at [get_robust_list(2)](https://man7.org/linux/man-pages/man2/get_robust_list.2.html).
    get_robust_list = 100,
    /// See doc at [nanosleep(2)](https://man7.org/linux/man-pages/man2/nanosleep.2.html).
    nanosleep = 101,
    /// See doc at [getitimer(2)](https://man7.org/linux/man-pages/man2/getitimer.2.html).
    getitimer = 102,
    /// See doc at [setitimer(2)](https://man7.org/linux/man-pages/man2/setitimer.2.html).
    setitimer = 103,
    /// See doc at [kexec_load(2)](https://man7.org/linux/man-pages/man2/kexec_load.2.html).
    kexec_load = 104,
    /// See doc at [init_module(2)](https://man7.org/linux/man-pages/man2/init_module.2.html).
    init_module = 105,
    /// See doc at [delete_module(2)](https://man7.org/linux/man-pages/man2/delete_module.2.html).
    delete_module = 106,
    /// See doc at [timer_create(2)](https://man7.org/linux/man-pages/man2/timer_create.2.html).
    timer_create = 107,
    /// See doc at [timer_gettime(2)](https://man7.org/linux/man-pages/man2/timer_gettime.2.html).
    timer_gettime = 108,
    /// See doc at [timer_getoverrun(2)](https://man7.org/linux/man-pages/man2/timer_getoverrun.2.html).
    timer_getoverrun = 109,
    /// See doc at [timer_settime(2)](https://man7.org/linux/man-pages/man2/timer_settime.2.html).
    timer_settime = 110,
    /// See doc at [timer_delete(2)](https://man7.org/linux/man-pages/man2/timer_delete.2.html).
    timer_delete = 111,
    /// See doc at [clock_settime(2)](https://man7.org/linux/man-pages/man2/clock_settime.2.html).
    clock_settime = 112,
    /// See doc at [clock_gettime(2)](https://man7.org/linux/man-pages/man2/clock_gettime.2.html).
    clock_gettime = 113,
    /// See doc at [clock_getres(2)](https://man7.org/linux/man-pages/man2/clock_getres.2.html).
    clock_getres = 114,
    /// See doc at [clock_nanosleep(2)](https://man7.org/linux/man-pages/man2/clock_nanosleep.2.html).
    clock_nanosleep = 115,
    /// See doc at [syslog(2)](https://man7.org/linux/man-pages/man2/syslog.2.html).
    syslog = 116,
    /// See doc at [ptrace(2)](https://man7.org/linux/man-pages/man2/ptrace.2.html).
    ptrace = 117,
    /// See doc at [sched_setparam(2)](https://man7.org/linux/man-pages/man2/sched_setparam.2.html).
    sched_setparam = 118,
    /// See doc at [sched_setscheduler(2)](https://man7.org/linux/man-pages/man2/sched_setscheduler.2.html).
    sched_setscheduler = 119,
    /// See doc at [sched_getscheduler(2)](https://man7.org/linux/man-pages/man2/sched_getscheduler.2.html).
    sched_getscheduler = 120,
    /// See doc at [sched_getparam(2)](https://man7.org/linux/man-pages/man2/sched_getparam.2.html).
    sched_getparam = 121,
    /// See doc at [sched_setaffinity(2)](https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html).
    sched_setaffinity = 122,
    /// See doc at [sched_getaffinity(2)](https://man7.org/linux/man-pages/man2/sched_getaffinity.2.html).
    sched_getaffinity = 123,
    /// See doc at [sched_yield(2)](https://man7.org/linux/man-pages/man2/sched_yield.2.html).
    sched_yield = 124,
    /// See doc at [sched_get_priority_max(2)](https://man7.org/linux/man-pages/man2/sched_get_priority_max.2.html).
    sched_get_priority_max = 125,
    /// See doc at [sched_get_priority_min(2)](https://man7.org/linux/man-pages/man2/sched_get_priority_min.2.html).
    sched_get_priority_min = 126,
    /// See doc at [sched_rr_get_interval(2)](https://man7.org/linux/man-pages/man2/sched_rr_get_interval.2.html).
    sched_rr_get_interval = 127,
    /// See doc at [restart_syscall(2)](https://man7.org/linux/man-pages/man2/restart_syscall.2.html).
    restart_syscall = 128,
    /// See doc at [kill(2)](https://man7.org/linux/man-pages/man2/kill.2.html).
    kill = 129,
    /// See doc at [tkill(2)](https://man7.org/linux/man-pages/man2/tkill.2.html).
    tkill = 130,
    /// See doc at [tgkill(2)](https://man7.org/linux/man-pages/man2/tgkill.2.html).
    tgkill = 131,
    /// See doc at [sigaltstack(2)](https://man7.org/linux/man-pages/man2/sigaltstack.2.html).
    sigaltstack = 132,
    /// See doc at [rt_sigsuspend(2)](https://man7.org/linux/man-pages/man2/rt_sigsuspend.2.html).
    rt_sigsuspend = 133,
    /// See doc at [rt_sigaction(2)](https://man7.org/linux/man-pages/man2/rt_sigaction.2.html).
    rt_sigaction = 134,
    /// See doc at [rt_sigprocmask(2)](https://man7.org/linux/man-pages/man2/rt_sigprocmask.2.html).
    rt_sigprocmask = 135,
    /// See doc at [rt_sigpending(2)](https://man7.org/linux/man-pages/man2/rt_sigpending.2.html).
    rt_sigpending = 136,
    /// See doc at [rt_sigtimedwait(2)](https://man7.org/linux/man-pages/man2/rt_sigtimedwait.2.html).
    rt_sigtimedwait = 137,
    /// See doc at [rt_sigqueueinfo(2)](https://man7.org/linux/man-pages/man2/rt_sigqueueinfo.2.html).
    rt_sigqueueinfo = 138,
    /// See doc at [rt_sigreturn(2)](https://man7.org/linux/man-pages/man2/rt_sigreturn.2.html).
    rt_sigreturn = 139,
    /// See doc at [setpriority(2)](https://man7.org/linux/man-pages/man2/setpriority.2.html).
    setpriority = 140,
    /// See doc at [getpriority(2)](https://man7.org/linux/man-pages/man2/getpriority.2.html).
    getpriority = 141,
    /// See doc at [reboot(2)](https://man7.org/linux/man-pages/man2/reboot.2.html).
    reboot = 142,
    /// See doc at [setregid(2)](https://man7.org/linux/man-pages/man2/setregid.2.html).
    setregid = 143,
    /// See doc at [setgid(2)](https://man7.org/linux/man-pages/man2/setgid.2.html).
    setgid = 144,
    /// See doc at [setreuid(2)](https://man7.org/linux/man-pages/man2/setreuid.2.html).
    setreuid = 145,
    /// See doc at [setuid(2)](https://man7.org/linux/man-pages/man2/setuid.2.html).
    setuid = 146,
    /// See doc at [setresuid(2)](https://man7.org/linux/man-pages/man2/setresuid.2.html).
    setresuid = 147,
    /// See doc at [getresuid(2)](https://man7.org/linux/man-pages/man2/getresuid.2.html).
    getresuid = 148,
    /// See doc at [setresgid(2)](https://man7.org/linux/man-pages/man2/setresgid.2.html).
    setresgid = 149,
    /// See doc at [getresgid(2)](https://man7.org/linux/man-pages/man2/getresgid.2.html).
    getresgid = 150,
    /// See doc at [setfsuid(2)](https://man7.org/linux/man-pages/man2/setfsuid.2.html).
    setfsuid = 151,
    /// See doc at [setfsgid(2)](https://man7.org/linux/man-pages/man2/setfsgid.2.html).
    setfsgid = 152,
    /// See doc at [times(2)](https://man7.org/linux/man-pages/man2/times.2.html).
    times = 153,
    /// See doc at [setpgid(2)](https://man7.org/linux/man-pages/man2/setpgid.2.html).
    setpgid = 154,
    /// See doc at [getpgid(2)](https://man7.org/linux/man-pages/man2/getpgid.2.html).
    getpgid = 155,
    /// See doc at [getsid(2)](https://man7.org/linux/man-pages/man2/getsid.2.html).
    getsid = 156,
    /// See doc at [setsid(2)](https://man7.org/linux/man-pages/man2/setsid.2.html).
    setsid = 157,
    /// See doc at [getgroups(2)](https://man7.org/linux/man-pages/man2/getgroups.2.html).
    getgroups = 158,
    /// See doc at [setgroups(2)](https://man7.org/linux/man-pages/man2/setgroups.2.html).
    setgroups = 159,
    /// See doc at [uname(2)](https://man7.org/linux/man-pages/man2/uname.2.html).
    uname = 160,
    /// See doc at [sethostname(2)](https://man7.org/linux/man-pages/man2/sethostname.2.html).
    sethostname = 161,
    /// See doc at [setdomainname(2)](https://man7.org/linux/man-pages/man2/setdomainname.2.html).
    setdomainname = 162,
    /// See doc at [getrlimit(2)](https://man7.org/linux/man-pages/man2/getrlimit.2.html).
    getrlimit = 163,
    /// See doc at [setrlimit(2)](https://man7.org/linux/man-pages/man2/setrlimit.2.html).
    setrlimit = 164,
    /// See doc at [getrusage(2)](https://man7.org/linux/man-pages/man2/getrusage.2.html).
    getrusage = 165,
    /// See doc at [umask(2)](https://man7.org/linux/man-pages/man2/umask.2.html).
    umask = 166,
    /// See doc at [prctl(2)](https://man7.org/linux/man-pages/man2/prctl.2.html).
    prctl = 167,
    /// See doc at [getcpu(2)](https://man7.org/linux/man-pages/man2/getcpu.2.html).
    getcpu = 168,
    /// See doc at [gettimeofday(2)](https://man7.org/linux/man-pages/man2/gettimeofday.2.html).
    gettimeofday = 169,
    /// See doc at [settimeofday(2)](https://man7.org/linux/man-pages/man2/settimeofday.2.html).
    settimeofday = 170,
    /// See doc at [adjtimex(2)](https://man7.org/linux/man-pages/man2/adjtimex.2.html).
    adjtimex = 171,
    /// See doc at [getpid(2)](https://man7.org/linux/man-pages/man2/getpid.2.html).
    getpid = 172,
    /// See doc at [getppid(2)](https://man7.org/linux/man-pages/man2/getppid.2.html).
    getppid = 173,
    /// See doc at [getuid(2)](https://man7.org/linux/man-pages/man2/getuid.2.html).
    getuid = 174,
    /// See doc at [geteuid(2)](https://man7.org/linux/man-pages/man2/geteuid.2.html).
    geteuid = 175,
    /// See doc at [getgid(2)](https://man7.org/linux/man-pages/man2/getgid.2.html).
    getgid = 176,
    /// See doc at [getegid(2)](https://man7.org/linux/man-pages/man2/getegid.2.html).
    getegid = 177,
    /// See doc at [gettid(2)](https://man7.org/linux/man-pages/man2/gettid.2.html).
    gettid = 178,
    /// See doc at [sysinfo(2)](https://man7.org/linux/man-pages/man2/sysinfo.2.html).
    sysinfo = 179,
    /// See doc at [mq_open(2)](https://man7.org/linux/man-pages/man2/mq_open.2.html).
    mq_open = 180,
    /// See doc at [mq_unlink(2)](https://man7.org/linux/man-pages/man2/mq_unlink.2.html).
    mq_unlink = 181,
    /// See doc at [mq_timedsend(2)](https://man7.org/linux/man-pages/man2/mq_timedsend.2.html).
    mq_timedsend = 182,
    /// See doc at [mq_timedreceive(2)](https://man7.org/linux/man-pages/man2/mq_timedreceive.2.html).
    mq_timedreceive = 183,
    /// See doc at [mq_notify(2)](https://man7.org/linux/man-pages/man2/mq_notify.2.html).
    mq_notify = 184,
    /// See doc at [mq_getsetattr(2)](https://man7.org/linux/man-pages/man2/mq_getsetattr.2.html).
    mq_getsetattr = 185,
    /// See doc at [msgget(2)](https://man7.org/linux/man-pages/man2/msgget.2.html).
    msgget = 186,
    /// See doc at [msgctl(2)](https://man7.org/linux/man-pages/man2/msgctl.2.html).
    msgctl = 187,
    /// See doc at [msgrcv(2)](https://man7.org/linux/man-pages/man2/msgrcv.2.html).
    msgrcv = 188,
    /// See doc at [msgsnd(2)](https://man7.org/linux/man-pages/man2/msgsnd.2.html).
    msgsnd = 189,
    /// See doc at [semget(2)](https://man7.org/linux/man-pages/man2/semget.2.html).
    semget = 190,
    /// See doc at [semctl(2)](https://man7.org/linux/man-pages/man2/semctl.2.html).
    semctl = 191,
    /// See doc at [semtimedop(2)](https://man7.org/linux/man-pages/man2/semtimedop.2.html).
    semtimedop = 192,
    /// See doc at [semop(2)](https://man7.org/linux/man-pages/man2/semop.2.html).
    semop = 193,
    /// See doc at [shmget(2)](https://man7.org/linux/man-pages/man2/shmget.2.html).
    shmget = 194,
    /// See doc at [shmctl(2)](https://man7.org/linux/man-pages/man2/shmctl.2.html).
    shmctl = 195,
    /// See doc at [shmat(2)](https://man7.org/linux/man-pages/man2/shmat.2.html).
    shmat = 196,
    /// See doc at [shmdt(2)](https://man7.org/linux/man-pages/man2/shmdt.2.html).
    shmdt = 197,
    /// See doc at [socket(2)](https://man7.org/linux/man-pages/man2/socket.2.html).
    socket = 198,
    /// See doc at [socketpair(2)](https://man7.org/linux/man-pages/man2/socketpair.2.html).
    socketpair = 199,
    /// See doc at [bind(2)](https://man7.org/linux/man-pages/man2/bind.2.html).
    bind = 200,
    /// See doc at [listen(2)](https://man7.org/linux/man-pages/man2/listen.2.html).
    listen = 201,
    /// See doc at [accept(2)](https://man7.org/linux/man-pages/man2/accept.2.html).
    accept = 202,
    /// See doc at [connect(2)](https://man7.org/linux/man-pages/man2/connect.2.html).
    connect = 203,
    /// See doc at [getsockname(2)](https://man7.org/linux/man-pages/man2/getsockname.2.html).
    getsockname = 204,
    /// See doc at [getpeername(2)](https://man7.org/linux/man-pages/man2/getpeername.2.html).
    getpeername = 205,
    /// See doc at [sendto(2)](https://man7.org/linux/man-pages/man2/sendto.2.html).
    sendto = 206,
    /// See doc at [recvfrom(2)](https://man7.org/linux/man-pages/man2/recvfrom.2.html).
    recvfrom = 207,
    /// See doc at [setsockopt(2)](https://man7.org/linux/man-pages/man2/setsockopt.2.html).
    setsockopt = 208,
    /// See doc at [getsockopt(2)](https://man7.org/linux/man-pages/man2/getsockopt.2.html).
    getsockopt = 209,
    /// See doc at [shutdown(2)](https://man7.org/linux/man-pages/man2/shutdown.2.html).
    shutdown = 210,
    /// See doc at [sendmsg(2)](https://man7.org/linux/man-pages/man2/sendmsg.2.html).
    sendmsg = 211,
    /// See doc at [recvmsg(2)](https://man7.org/linux/man-pages/man2/recvmsg.2.html).
    recvmsg = 212,
    /// See doc at [readahead(2)](https://man7.org/linux/man-pages/man2/readahead.2.html).
    readahead = 213,
    /// See doc at [brk(2)](https://man7.org/linux/man-pages/man2/brk.2.html).
    brk = 214,
    /// See doc at [munmap(2)](https://man7.org/linux/man-pages/man2/munmap.2.html).
    munmap = 215,
    /// See doc at [mremap(2)](https://man7.org/linux/man-pages/man2/mremap.2.html).
    mremap = 216,
    /// See doc at [add_key(2)](https://man7.org/linux/man-pages/man2/add_key.2.html).
    add_key = 217,
    /// See doc at [request_key(2)](https://man7.org/linux/man-pages/man2/request_key.2.html).
    request_key = 218,
    /// See doc at [keyctl(2)](https://man7.org/linux/man-pages/man2/keyctl.2.html).
    keyctl = 219,
    /// See doc at [clone(2)](https://man7.org/linux/man-pages/man2/clone.2.html).
    clone = 220,
    /// See doc at [execve(2)](https://man7.org/linux/man-pages/man2/execve.2.html).
    execve = 221,
    /// See doc at [mmap(2)](https://man7.org/linux/man-pages/man2/mmap.2.html).
    mmap = 222,
    /// See doc at [fadvise64(2)](https://man7.org/linux/man-pages/man2/fadvise64.2.html).
    fadvise64 = 223,
    /// See doc at [swapon(2)](https://man7.org/linux/man-pages/man2/swapon.2.html).
    swapon = 224,
    /// See doc at [swapoff(2)](https://man7.org/linux/man-pages/man2/swapoff.2.html).
    swapoff = 225,
    /// See doc at [mprotect(2)](https://man7.org/linux/man-pages/man2/mprotect.2.html).
    mprotect = 226,
    /// See doc at [msync(2)](https://man7.org/linux/man-pages/man2/msync.2.html).
    msync = 227,
    /// See doc at [mlock(2)](https://man7.org/linux/man-pages/man2/mlock.2.html).
    mlock = 228,
    /// See doc at [munlock(2)](https://man7.org/linux/man-pages/man2/munlock.2.html).
    munlock = 229,
    /// See doc at [mlockall(2)](https://man7.org/linux/man-pages/man2/mlockall.2.html).
    mlockall = 230,
    /// See doc at [munlockall(2)](https://man7.org/linux/man-pages/man2/munlockall.2.html).
    munlockall = 231,
    /// See doc at [mincore(2)](https://man7.org/linux/man-pages/man2/mincore.2.html).
    mincore = 232,
    /// See doc at [madvise(2)](https://man7.org/linux/man-pages/man2/madvise.2.html).
    madvise = 233,
    /// See doc at [remap_file_pages(2)](https://man7.org/linux/man-pages/man2/remap_file_pages.2.html).
    remap_file_pages = 234,
    /// See doc at [mbind(2)](https://man7.org/linux/man-pages/man2/mbind.2.html).
    mbind = 235,
    /// See doc at [get_mempolicy(2)](https://man7.org/linux/man-pages/man2/get_mempolicy.2.html).
    get_mempolicy = 236,
    /// See doc at [set_mempolicy(2)](https://man7.org/linux/man-pages/man2/set_mempolicy.2.html).
    set_mempolicy = 237,
    /// See doc at [migrate_pages(2)](https://man7.org/linux/man-pages/man2/migrate_pages.2.html).
    migrate_pages = 238,
    /// See doc at [move_pages(2)](https://man7.org/linux/man-pages/man2/move_pages.2.html).
    move_pages = 239,
    /// See doc at [rt_tgsigqueueinfo(2)](https://man7.org/linux/man-pages/man2/rt_tgsigqueueinfo.2.html).
    rt_tgsigqueueinfo = 240,
    /// See doc at [perf_event_open(2)](https://man7.org/linux/man-pages/man2/perf_event_open.2.html).
    perf_event_open = 241,
    /// See doc at [accept4(2)](https://man7.org/linux/man-pages/man2/accept4.2.html).
    accept4 = 242,
    /// See doc at [recvmmsg(2)](https://man7.org/linux/man-pages/man2/recvmmsg.2.html).
    recvmmsg = 243,
    /// See doc at [wait4(2)](https://man7.org/linux/man-pages/man2/wait4.2.html).
    wait4 = 260,
    /// See doc at [prlimit64(2)](https://man7.org/linux/man-pages/man2/prlimit64.2.html).
    prlimit64 = 261,
    /// See doc at [fanotify_init(2)](https://man7.org/linux/man-pages/man2/fanotify_init.2.html).
    fanotify_init = 262,
    /// See doc at [fanotify_mark(2)](https://man7.org/linux/man-pages/man2/fanotify_mark.2.html).
    fanotify_mark = 263,
    /// See doc at [name_to_handle_at(2)](https://man7.org/linux/man-pages/man2/name_to_handle_at.2.html).
    name_to_handle_at = 264,
    /// See doc at [open_by_handle_at(2)](https://man7.org/linux/man-pages/man2/open_by_handle_at.2.html).
    open_by_handle_at = 265,
    /// See doc at [clock_adjtime(2)](https://man7.org/linux/man-pages/man2/clock_adjtime.2.html).
    clock_adjtime = 266,
    /// See doc at [syncfs(2)](https://man7.org/linux/man-pages/man2/syncfs.2.html).
    syncfs = 267,
    /// See doc at [setns(2)](https://man7.org/linux/man-pages/man2/setns.2.html).
    setns = 268,
    /// See doc at [sendmmsg(2)](https://man7.org/linux/man-pages/man2/sendmmsg.2.html).
    sendmmsg = 269,
    /// See doc at [process_vm_readv(2)](https://man7.org/linux/man-pages/man2/process_vm_readv.2.html).
    process_vm_readv = 270,
    /// See doc at [process_vm_writev(2)](https://man7.org/linux/man-pages/man2/process_vm_writev.2.html).
    process_vm_writev = 271,
    /// See doc at [kcmp(2)](https://man7.org/linux/man-pages/man2/kcmp.2.html).
    kcmp = 272,
    /// See doc at [finit_module(2)](https://man7.org/linux/man-pages/man2/finit_module.2.html).
    finit_module = 273,
    /// See doc at [sched_setattr(2)](https://man7.org/linux/man-pages/man2/sched_setattr.2.html).
    sched_setattr = 274,
    /// See doc at [sched_getattr(2)](https://man7.org/linux/man-pages/man2/sched_getattr.2.html).
    sched_getattr = 275,
    /// See doc at [renameat2(2)](https://man7.org/linux/man-pages/man2/renameat2.2.html).
    renameat2 = 276,
    /// See doc at [seccomp(2)](https://man7.org/linux/man-pages/man2/seccomp.2.html).
    seccomp = 277,
    /// See doc at [getrandom(2)](https://man7.org/linux/man-pages/man2/getrandom.2.html).
    getrandom = 278,
    /// See doc at [memfd_create(2)](https://man7.org/linux/man-pages/man2/memfd_create.2.html).
    memfd_create = 279,
    /// See doc at [bpf(2)](https://man7.org/linux/man-pages/man2/bpf.2.html).
    bpf = 280,
    /// See doc at [execveat(2)](https://man7.org/linux/man-pages/man2/execveat.2.html).
    execveat = 281,
    /// See doc at [userfaultfd(2)](https://man7.org/linux/man-pages/man2/userfaultfd.2.html).
    userfaultfd = 282,
    /// See doc at [membarrier(2)](https://man7.org/linux/man-pages/man2/membarrier.2.html).
    membarrier = 283,
    /// See doc at [mlock2(2)](https://man7.org/linux/man-pages/man2/mlock2.2.html).
    mlock2 = 284,
    /// See doc at [copy_file_range(2)](https://man7.org/linux/man-pages/man2/copy_file_range.2.html).
    copy_file_range = 285,
    /// See doc at [preadv2(2)](https://man7.org/linux/man-pages/man2/preadv2.2.html).
    preadv2 = 286,
    /// See doc at [pwritev2(2)](https://man7.org/linux/man-pages/man2/pwritev2.2.html).
    pwritev2 = 287,
    /// See doc at [pkey_mprotect(2)](https://man7.org/linux/man-pages/man2/pkey_mprotect.2.html).
    pkey_mprotect = 288,
    /// See doc at [pkey_alloc(2)](https://man7.org/linux/man-pages/man2/pkey_alloc.2.html).
    pkey_alloc = 289,
    /// See doc at [pkey_free(2)](https://man7.org/linux/man-pages/man2/pkey_free.2.html).
    pkey_free = 290,
    /// See doc at [statx(2)](https://man7.org/linux/man-pages/man2/statx.2.html).
    statx = 291,
    /// See doc at [io_pgetevents(2)](https://man7.org/linux/man-pages/man2/io_pgetevents.2.html).
    io_pgetevents = 292,
    /// See doc at [rseq(2)](https://man7.org/linux/man-pages/man2/rseq.2.html).
    rseq = 293,
    /// See doc at [kexec_file_load(2)](https://man7.org/linux/man-pages/man2/kexec_file_load.2.html).
    kexec_file_load = 294,
    /// See doc at [clock_gettime64(2)](https://man7.org/linux/man-pages/man2/clock_gettime64.2.html).
    clock_gettime64 = 403,
    /// See doc at [clock_settime64(2)](https://man7.org/linux/man-pages/man2/clock_settime64.2.html).
    clock_settime64 = 404,
    /// See doc at [clock_adjtime64(2)](https://man7.org/linux/man-pages/man2/clock_adjtime64.2.html).
    clock_adjtime64 = 405,
    /// See doc at [clock_getres_time64(2)](https://man7.org/linux/man-pages/man2/clock_getres_time64.2.html).
    clock_getres_time64 = 406,
    /// See doc at [clock_nanosleep_time64(2)](https://man7.org/linux/man-pages/man2/clock_nanosleep_time64.2.html).
    clock_nanosleep_time64 = 407,
    /// See doc at [timer_gettime64(2)](https://man7.org/linux/man-pages/man2/timer_gettime64.2.html).
    timer_gettime64 = 408,
    /// See doc at [timer_settime64(2)](https://man7.org/linux/man-pages/man2/timer_settime64.2.html).
    timer_settime64 = 409,
    /// See doc at [timerfd_gettime64(2)](https://man7.org/linux/man-pages/man2/timerfd_gettime64.2.html).
    timerfd_gettime64 = 410,
    /// See doc at [timerfd_settime64(2)](https://man7.org/linux/man-pages/man2/timerfd_settime64.2.html).
    timerfd_settime64 = 411,
    /// See doc at [utimensat_time64(2)](https://man7.org/linux/man-pages/man2/utimensat_time64.2.html).
    utimensat_time64 = 412,
    /// See doc at [pselect6_time64(2)](https://man7.org/linux/man-pages/man2/pselect6_time64.2.html).
    pselect6_time64 = 413,
    /// See doc at [ppoll_time64(2)](https://man7.org/linux/man-pages/man2/ppoll_time64.2.html).
    ppoll_time64 = 414,
    /// See doc at [io_pgetevents_time64(2)](https://man7.org/linux/man-pages/man2/io_pgetevents_time64.2.html).
    io_pgetevents_time64 = 416,
    /// See doc at [recvmmsg_time64(2)](https://man7.org/linux/man-pages/man2/recvmmsg_time64.2.html).
    recvmmsg_time64 = 417,
    /// See doc at [mq_timedsend_time64(2)](https://man7.org/linux/man-pages/man2/mq_timedsend_time64.2.html).
    mq_timedsend_time64 = 418,
    /// See doc at [mq_timedreceive_time64(2)](https://man7.org/linux/man-pages/man2/mq_timedreceive_time64.2.html).
    mq_timedreceive_time64 = 419,
    /// See doc at [semtimedop_time64(2)](https://man7.org/linux/man-pages/man2/semtimedop_time64.2.html).
    semtimedop_time64 = 420,
    /// See doc at [rt_sigtimedwait_time64(2)](https://man7.org/linux/man-pages/man2/rt_sigtimedwait_time64.2.html).
    rt_sigtimedwait_time64 = 421,
    /// See doc at [futex_time64(2)](https://man7.org/linux/man-pages/man2/futex_time64.2.html).
    futex_time64 = 422,
    /// See doc at [sched_rr_get_interval_time64(2)](https://man7.org/linux/man-pages/man2/sched_rr_get_interval_time64.2.html).
    sched_rr_get_interval_time64 = 423,
    /// See doc at [pidfd_send_signal(2)](https://man7.org/linux/man-pages/man2/pidfd_send_signal.2.html).
    pidfd_send_signal = 424,
    /// See doc at [io_uring_setup(2)](https://man7.org/linux/man-pages/man2/io_uring_setup.2.html).
    io_uring_setup = 425,
    /// See doc at [io_uring_enter(2)](https://man7.org/linux/man-pages/man2/io_uring_enter.2.html).
    io_uring_enter = 426,
    /// See doc at [io_uring_register(2)](https://man7.org/linux/man-pages/man2/io_uring_register.2.html).
    io_uring_register = 427,
    /// See doc at [open_tree(2)](https://man7.org/linux/man-pages/man2/open_tree.2.html).
    open_tree = 428,
    /// See doc at [move_mount(2)](https://man7.org/linux/man-pages/man2/move_mount.2.html).
    move_mount = 429,
    /// See doc at [fsopen(2)](https://man7.org/linux/man-pages/man2/fsopen.2.html).
    fsopen = 430,
    /// See doc at [fsconfig(2)](https://man7.org/linux/man-pages/man2/fsconfig.2.html).
    fsconfig = 431,
    /// See doc at [fsmount(2)](https://man7.org/linux/man-pages/man2/fsmount.2.html).
    fsmount = 432,
    /// See doc at [fspick(2)](https://man7.org/linux/man-pages/man2/fspick.2.html).
    fspick = 433,
    /// See doc at [pidfd_open(2)](https://man7.org/linux/man-pages/man2/pidfd_open.2.html).
    pidfd_open = 434,
    /// See doc at [clone3(2)](https://man7.org/linux/man-pages/man2/clone3.2.html).
    clone3 = 435,
    /// See doc at [close_range(2)](https://man7.org/linux/man-pages/man2/close_range.2.html).
    close_range = 436,
    /// See doc at [openat2(2)](https://man7.org/linux/man-pages/man2/openat2.2.html).
    openat2 = 437,
    /// See doc at [pidfd_getfd(2)](https://man7.org/linux/man-pages/man2/pidfd_getfd.2.html).
    pidfd_getfd = 438,
    /// See doc at [faccessat2(2)](https://man7.org/linux/man-pages/man2/faccessat2.2.html).
    faccessat2 = 439,
    /// See doc at [process_madvise(2)](https://man7.org/linux/man-pages/man2/process_madvise.2.html).
    process_madvise = 440,
    /// See doc at [epoll_pwait2(2)](https://man7.org/linux/man-pages/man2/epoll_pwait2.2.html).
    epoll_pwait2 = 441,
    /// See doc at [mount_setattr(2)](https://man7.org/linux/man-pages/man2/mount_setattr.2.html).
    mount_setattr = 442,
    /// See doc at [quotactl_fd(2)](https://man7.org/linux/man-pages/man2/quotactl_fd.2.html).
    quotactl_fd = 443,
    /// See doc at [landlock_create_ruleset(2)](https://man7.org/linux/man-pages/man2/landlock_create_ruleset.2.html).
    landlock_create_ruleset = 444,
    /// See doc at [landlock_add_rule(2)](https://man7.org/linux/man-pages/man2/landlock_add_rule.2.html).
    landlock_add_rule = 445,
    /// See doc at [landlock_restrict_self(2)](https://man7.org/linux/man-pages/man2/landlock_restrict_self.2.html).
    landlock_restrict_self = 446,
    /// See doc at [memfd_secret(2)](https://man7.org/linux/man-pages/man2/memfd_secret.2.html).
    memfd_secret = 447,
    /// See doc at [process_mrelease(2)](https://man7.org/linux/man-pages/man2/process_mrelease.2.html).
    process_mrelease = 448,
    /// See doc at [futex_waitv(2)](https://man7.org/linux/man-pages/man2/futex_waitv.2.html).
    futex_waitv = 449,
    /// See doc at [set_mempolicy_home_node(2)](https://man7.org/linux/man-pages/man2/set_mempolicy_home_node.2.html).
    set_mempolicy_home_node = 450,
    /// See doc at [cacheflush(2)](https://man7.org/linux/man-pages/man2/cacheflush.2.html).
    cacheflush = 983042,
    /// See doc at [set_tls(2)](https://man7.org/linux/man-pages/man2/set_tls.2.html).
    set_tls = 983045,
}
