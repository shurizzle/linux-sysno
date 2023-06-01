#![allow(non_camel_case_types)]

// This file is automatically generated. Do not edit.

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sysno {
    read = 1073741824,
    write = 1073741825,
    open = 1073741826,
    close = 1073741827,
    stat = 1073741828,
    fstat = 1073741829,
    lstat = 1073741830,
    poll = 1073741831,
    lseek = 1073741832,
    mmap = 1073741833,
    mprotect = 1073741834,
    munmap = 1073741835,
    brk = 1073741836,
    rt_sigprocmask = 1073741838,
    pread64 = 1073741841,
    pwrite64 = 1073741842,
    access = 1073741845,
    pipe = 1073741846,
    select = 1073741847,
    sched_yield = 1073741848,
    mremap = 1073741849,
    msync = 1073741850,
    mincore = 1073741851,
    madvise = 1073741852,
    shmget = 1073741853,
    shmat = 1073741854,
    shmctl = 1073741855,
    dup = 1073741856,
    dup2 = 1073741857,
    pause = 1073741858,
    nanosleep = 1073741859,
    getitimer = 1073741860,
    alarm = 1073741861,
    setitimer = 1073741862,
    getpid = 1073741863,
    sendfile = 1073741864,
    socket = 1073741865,
    connect = 1073741866,
    accept = 1073741867,
    sendto = 1073741868,
    shutdown = 1073741872,
    bind = 1073741873,
    listen = 1073741874,
    getsockname = 1073741875,
    getpeername = 1073741876,
    socketpair = 1073741877,
    clone = 1073741880,
    fork = 1073741881,
    vfork = 1073741882,
    exit = 1073741884,
    wait4 = 1073741885,
    kill = 1073741886,
    uname = 1073741887,
    semget = 1073741888,
    semop = 1073741889,
    semctl = 1073741890,
    shmdt = 1073741891,
    msgget = 1073741892,
    msgsnd = 1073741893,
    msgrcv = 1073741894,
    msgctl = 1073741895,
    fcntl = 1073741896,
    flock = 1073741897,
    fsync = 1073741898,
    fdatasync = 1073741899,
    truncate = 1073741900,
    ftruncate = 1073741901,
    getdents = 1073741902,
    getcwd = 1073741903,
    chdir = 1073741904,
    fchdir = 1073741905,
    rename = 1073741906,
    mkdir = 1073741907,
    rmdir = 1073741908,
    creat = 1073741909,
    link = 1073741910,
    unlink = 1073741911,
    symlink = 1073741912,
    readlink = 1073741913,
    chmod = 1073741914,
    fchmod = 1073741915,
    chown = 1073741916,
    fchown = 1073741917,
    lchown = 1073741918,
    umask = 1073741919,
    gettimeofday = 1073741920,
    getrlimit = 1073741921,
    getrusage = 1073741922,
    sysinfo = 1073741923,
    times = 1073741924,
    getuid = 1073741926,
    syslog = 1073741927,
    getgid = 1073741928,
    setuid = 1073741929,
    setgid = 1073741930,
    geteuid = 1073741931,
    getegid = 1073741932,
    setpgid = 1073741933,
    getppid = 1073741934,
    getpgrp = 1073741935,
    setsid = 1073741936,
    setreuid = 1073741937,
    setregid = 1073741938,
    getgroups = 1073741939,
    setgroups = 1073741940,
    setresuid = 1073741941,
    getresuid = 1073741942,
    setresgid = 1073741943,
    getresgid = 1073741944,
    getpgid = 1073741945,
    setfsuid = 1073741946,
    setfsgid = 1073741947,
    getsid = 1073741948,
    capget = 1073741949,
    capset = 1073741950,
    rt_sigsuspend = 1073741954,
    utime = 1073741956,
    mknod = 1073741957,
    personality = 1073741959,
    ustat = 1073741960,
    statfs = 1073741961,
    fstatfs = 1073741962,
    sysfs = 1073741963,
    getpriority = 1073741964,
    setpriority = 1073741965,
    sched_setparam = 1073741966,
    sched_getparam = 1073741967,
    sched_setscheduler = 1073741968,
    sched_getscheduler = 1073741969,
    sched_get_priority_max = 1073741970,
    sched_get_priority_min = 1073741971,
    sched_rr_get_interval = 1073741972,
    mlock = 1073741973,
    munlock = 1073741974,
    mlockall = 1073741975,
    munlockall = 1073741976,
    vhangup = 1073741977,
    modify_ldt = 1073741978,
    pivot_root = 1073741979,
    prctl = 1073741981,
    arch_prctl = 1073741982,
    adjtimex = 1073741983,
    setrlimit = 1073741984,
    chroot = 1073741985,
    sync = 1073741986,
    acct = 1073741987,
    settimeofday = 1073741988,
    mount = 1073741989,
    umount2 = 1073741990,
    swapon = 1073741991,
    swapoff = 1073741992,
    reboot = 1073741993,
    sethostname = 1073741994,
    setdomainname = 1073741995,
    iopl = 1073741996,
    ioperm = 1073741997,
    init_module = 1073741999,
    delete_module = 1073742000,
    quotactl = 1073742003,
    getpmsg = 1073742005,
    putpmsg = 1073742006,
    afs_syscall = 1073742007,
    tuxcall = 1073742008,
    security = 1073742009,
    gettid = 1073742010,
    readahead = 1073742011,
    setxattr = 1073742012,
    lsetxattr = 1073742013,
    fsetxattr = 1073742014,
    getxattr = 1073742015,
    lgetxattr = 1073742016,
    fgetxattr = 1073742017,
    listxattr = 1073742018,
    llistxattr = 1073742019,
    flistxattr = 1073742020,
    removexattr = 1073742021,
    lremovexattr = 1073742022,
    fremovexattr = 1073742023,
    tkill = 1073742024,
    time = 1073742025,
    futex = 1073742026,
    sched_setaffinity = 1073742027,
    sched_getaffinity = 1073742028,
    io_destroy = 1073742031,
    io_getevents = 1073742032,
    io_cancel = 1073742034,
    lookup_dcookie = 1073742036,
    epoll_create = 1073742037,
    remap_file_pages = 1073742040,
    getdents64 = 1073742041,
    set_tid_address = 1073742042,
    restart_syscall = 1073742043,
    semtimedop = 1073742044,
    fadvise64 = 1073742045,
    timer_settime = 1073742047,
    timer_gettime = 1073742048,
    timer_getoverrun = 1073742049,
    timer_delete = 1073742050,
    clock_settime = 1073742051,
    clock_gettime = 1073742052,
    clock_getres = 1073742053,
    clock_nanosleep = 1073742054,
    exit_group = 1073742055,
    epoll_wait = 1073742056,
    epoll_ctl = 1073742057,
    tgkill = 1073742058,
    utimes = 1073742059,
    mbind = 1073742061,
    set_mempolicy = 1073742062,
    get_mempolicy = 1073742063,
    mq_open = 1073742064,
    mq_unlink = 1073742065,
    mq_timedsend = 1073742066,
    mq_timedreceive = 1073742067,
    mq_getsetattr = 1073742069,
    add_key = 1073742072,
    request_key = 1073742073,
    keyctl = 1073742074,
    ioprio_set = 1073742075,
    ioprio_get = 1073742076,
    inotify_init = 1073742077,
    inotify_add_watch = 1073742078,
    inotify_rm_watch = 1073742079,
    migrate_pages = 1073742080,
    openat = 1073742081,
    mkdirat = 1073742082,
    mknodat = 1073742083,
    fchownat = 1073742084,
    futimesat = 1073742085,
    newfstatat = 1073742086,
    unlinkat = 1073742087,
    renameat = 1073742088,
    linkat = 1073742089,
    symlinkat = 1073742090,
    readlinkat = 1073742091,
    fchmodat = 1073742092,
    faccessat = 1073742093,
    pselect6 = 1073742094,
    ppoll = 1073742095,
    unshare = 1073742096,
    splice = 1073742099,
    tee = 1073742100,
    sync_file_range = 1073742101,
    utimensat = 1073742104,
    epoll_pwait = 1073742105,
    signalfd = 1073742106,
    timerfd_create = 1073742107,
    eventfd = 1073742108,
    fallocate = 1073742109,
    timerfd_settime = 1073742110,
    timerfd_gettime = 1073742111,
    accept4 = 1073742112,
    signalfd4 = 1073742113,
    eventfd2 = 1073742114,
    epoll_create1 = 1073742115,
    dup3 = 1073742116,
    pipe2 = 1073742117,
    inotify_init1 = 1073742118,
    perf_event_open = 1073742122,
    fanotify_init = 1073742124,
    fanotify_mark = 1073742125,
    prlimit64 = 1073742126,
    name_to_handle_at = 1073742127,
    open_by_handle_at = 1073742128,
    clock_adjtime = 1073742129,
    syncfs = 1073742130,
    setns = 1073742132,
    getcpu = 1073742133,
    kcmp = 1073742136,
    finit_module = 1073742137,
    sched_setattr = 1073742138,
    sched_getattr = 1073742139,
    renameat2 = 1073742140,
    seccomp = 1073742141,
    getrandom = 1073742142,
    memfd_create = 1073742143,
    kexec_file_load = 1073742144,
    bpf = 1073742145,
    userfaultfd = 1073742147,
    membarrier = 1073742148,
    mlock2 = 1073742149,
    copy_file_range = 1073742150,
    pkey_mprotect = 1073742153,
    pkey_alloc = 1073742154,
    pkey_free = 1073742155,
    statx = 1073742156,
    io_pgetevents = 1073742157,
    rseq = 1073742158,
    pidfd_send_signal = 1073742248,
    io_uring_setup = 1073742249,
    io_uring_enter = 1073742250,
    io_uring_register = 1073742251,
    open_tree = 1073742252,
    move_mount = 1073742253,
    fsopen = 1073742254,
    fsconfig = 1073742255,
    fsmount = 1073742256,
    fspick = 1073742257,
    pidfd_open = 1073742258,
    clone3 = 1073742259,
    close_range = 1073742260,
    openat2 = 1073742261,
    pidfd_getfd = 1073742262,
    faccessat2 = 1073742263,
    process_madvise = 1073742264,
    epoll_pwait2 = 1073742265,
    mount_setattr = 1073742266,
    quotactl_fd = 1073742267,
    landlock_create_ruleset = 1073742268,
    landlock_add_rule = 1073742269,
    landlock_restrict_self = 1073742270,
    memfd_secret = 1073742271,
    process_mrelease = 1073742272,
    futex_waitv = 1073742273,
    set_mempolicy_home_node = 1073742274,
    rt_sigaction = 1073742336,
    rt_sigreturn = 1073742337,
    ioctl = 1073742338,
    readv = 1073742339,
    writev = 1073742340,
    recvfrom = 1073742341,
    sendmsg = 1073742342,
    recvmsg = 1073742343,
    execve = 1073742344,
    ptrace = 1073742345,
    rt_sigpending = 1073742346,
    rt_sigtimedwait = 1073742347,
    rt_sigqueueinfo = 1073742348,
    sigaltstack = 1073742349,
    timer_create = 1073742350,
    mq_notify = 1073742351,
    kexec_load = 1073742352,
    waitid = 1073742353,
    set_robust_list = 1073742354,
    get_robust_list = 1073742355,
    vmsplice = 1073742356,
    move_pages = 1073742357,
    preadv = 1073742358,
    pwritev = 1073742359,
    rt_tgsigqueueinfo = 1073742360,
    recvmmsg = 1073742361,
    sendmmsg = 1073742362,
    process_vm_readv = 1073742363,
    process_vm_writev = 1073742364,
    setsockopt = 1073742365,
    getsockopt = 1073742366,
    io_setup = 1073742367,
    io_submit = 1073742368,
    execveat = 1073742369,
    preadv2 = 1073742370,
    pwritev2 = 1073742371,
}
