#![allow(non_camel_case_types)]

// This file is automatically generated. Do not edit.

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sysno {
    read = 5000,
    write = 5001,
    open = 5002,
    close = 5003,
    stat = 5004,
    fstat = 5005,
    lstat = 5006,
    poll = 5007,
    lseek = 5008,
    mmap = 5009,
    mprotect = 5010,
    munmap = 5011,
    brk = 5012,
    rt_sigaction = 5013,
    rt_sigprocmask = 5014,
    ioctl = 5015,
    pread64 = 5016,
    pwrite64 = 5017,
    readv = 5018,
    writev = 5019,
    access = 5020,
    pipe = 5021,
    _newselect = 5022,
    sched_yield = 5023,
    mremap = 5024,
    msync = 5025,
    mincore = 5026,
    madvise = 5027,
    shmget = 5028,
    shmat = 5029,
    shmctl = 5030,
    dup = 5031,
    dup2 = 5032,
    pause = 5033,
    nanosleep = 5034,
    getitimer = 5035,
    setitimer = 5036,
    alarm = 5037,
    getpid = 5038,
    sendfile = 5039,
    socket = 5040,
    connect = 5041,
    accept = 5042,
    sendto = 5043,
    recvfrom = 5044,
    sendmsg = 5045,
    recvmsg = 5046,
    shutdown = 5047,
    bind = 5048,
    listen = 5049,
    getsockname = 5050,
    getpeername = 5051,
    socketpair = 5052,
    setsockopt = 5053,
    getsockopt = 5054,
    clone = 5055,
    fork = 5056,
    execve = 5057,
    exit = 5058,
    wait4 = 5059,
    kill = 5060,
    uname = 5061,
    semget = 5062,
    semop = 5063,
    semctl = 5064,
    shmdt = 5065,
    msgget = 5066,
    msgsnd = 5067,
    msgrcv = 5068,
    msgctl = 5069,
    fcntl = 5070,
    flock = 5071,
    fsync = 5072,
    fdatasync = 5073,
    truncate = 5074,
    ftruncate = 5075,
    getdents = 5076,
    getcwd = 5077,
    chdir = 5078,
    fchdir = 5079,
    rename = 5080,
    mkdir = 5081,
    rmdir = 5082,
    creat = 5083,
    link = 5084,
    unlink = 5085,
    symlink = 5086,
    readlink = 5087,
    chmod = 5088,
    fchmod = 5089,
    chown = 5090,
    fchown = 5091,
    lchown = 5092,
    umask = 5093,
    gettimeofday = 5094,
    getrlimit = 5095,
    getrusage = 5096,
    sysinfo = 5097,
    times = 5098,
    ptrace = 5099,
    getuid = 5100,
    syslog = 5101,
    getgid = 5102,
    setuid = 5103,
    setgid = 5104,
    geteuid = 5105,
    getegid = 5106,
    setpgid = 5107,
    getppid = 5108,
    getpgrp = 5109,
    setsid = 5110,
    setreuid = 5111,
    setregid = 5112,
    getgroups = 5113,
    setgroups = 5114,
    setresuid = 5115,
    getresuid = 5116,
    setresgid = 5117,
    getresgid = 5118,
    getpgid = 5119,
    setfsuid = 5120,
    setfsgid = 5121,
    getsid = 5122,
    capget = 5123,
    capset = 5124,
    rt_sigpending = 5125,
    rt_sigtimedwait = 5126,
    rt_sigqueueinfo = 5127,
    rt_sigsuspend = 5128,
    sigaltstack = 5129,
    utime = 5130,
    mknod = 5131,
    personality = 5132,
    ustat = 5133,
    statfs = 5134,
    fstatfs = 5135,
    sysfs = 5136,
    getpriority = 5137,
    setpriority = 5138,
    sched_setparam = 5139,
    sched_getparam = 5140,
    sched_setscheduler = 5141,
    sched_getscheduler = 5142,
    sched_get_priority_max = 5143,
    sched_get_priority_min = 5144,
    sched_rr_get_interval = 5145,
    mlock = 5146,
    munlock = 5147,
    mlockall = 5148,
    munlockall = 5149,
    vhangup = 5150,
    pivot_root = 5151,
    _sysctl = 5152,
    prctl = 5153,
    adjtimex = 5154,
    setrlimit = 5155,
    chroot = 5156,
    sync = 5157,
    acct = 5158,
    settimeofday = 5159,
    mount = 5160,
    umount2 = 5161,
    swapon = 5162,
    swapoff = 5163,
    reboot = 5164,
    sethostname = 5165,
    setdomainname = 5166,
    create_module = 5167,
    init_module = 5168,
    delete_module = 5169,
    get_kernel_syms = 5170,
    query_module = 5171,
    quotactl = 5172,
    nfsservctl = 5173,
    getpmsg = 5174,
    putpmsg = 5175,
    afs_syscall = 5176,
    reserved177 = 5177,
    gettid = 5178,
    readahead = 5179,
    setxattr = 5180,
    lsetxattr = 5181,
    fsetxattr = 5182,
    getxattr = 5183,
    lgetxattr = 5184,
    fgetxattr = 5185,
    listxattr = 5186,
    llistxattr = 5187,
    flistxattr = 5188,
    removexattr = 5189,
    lremovexattr = 5190,
    fremovexattr = 5191,
    tkill = 5192,
    reserved193 = 5193,
    futex = 5194,
    sched_setaffinity = 5195,
    sched_getaffinity = 5196,
    cacheflush = 5197,
    cachectl = 5198,
    sysmips = 5199,
    io_setup = 5200,
    io_destroy = 5201,
    io_getevents = 5202,
    io_submit = 5203,
    io_cancel = 5204,
    exit_group = 5205,
    lookup_dcookie = 5206,
    epoll_create = 5207,
    epoll_ctl = 5208,
    epoll_wait = 5209,
    remap_file_pages = 5210,
    rt_sigreturn = 5211,
    set_tid_address = 5212,
    restart_syscall = 5213,
    semtimedop = 5214,
    fadvise64 = 5215,
    timer_create = 5216,
    timer_settime = 5217,
    timer_gettime = 5218,
    timer_getoverrun = 5219,
    timer_delete = 5220,
    clock_settime = 5221,
    clock_gettime = 5222,
    clock_getres = 5223,
    clock_nanosleep = 5224,
    tgkill = 5225,
    utimes = 5226,
    mbind = 5227,
    get_mempolicy = 5228,
    set_mempolicy = 5229,
    mq_open = 5230,
    mq_unlink = 5231,
    mq_timedsend = 5232,
    mq_timedreceive = 5233,
    mq_notify = 5234,
    mq_getsetattr = 5235,
    vserver = 5236,
    waitid = 5237,
    add_key = 5239,
    request_key = 5240,
    keyctl = 5241,
    set_thread_area = 5242,
    inotify_init = 5243,
    inotify_add_watch = 5244,
    inotify_rm_watch = 5245,
    migrate_pages = 5246,
    openat = 5247,
    mkdirat = 5248,
    mknodat = 5249,
    fchownat = 5250,
    futimesat = 5251,
    newfstatat = 5252,
    unlinkat = 5253,
    renameat = 5254,
    linkat = 5255,
    symlinkat = 5256,
    readlinkat = 5257,
    fchmodat = 5258,
    faccessat = 5259,
    pselect6 = 5260,
    ppoll = 5261,
    unshare = 5262,
    splice = 5263,
    sync_file_range = 5264,
    tee = 5265,
    vmsplice = 5266,
    move_pages = 5267,
    set_robust_list = 5268,
    get_robust_list = 5269,
    kexec_load = 5270,
    getcpu = 5271,
    epoll_pwait = 5272,
    ioprio_set = 5273,
    ioprio_get = 5274,
    utimensat = 5275,
    signalfd = 5276,
    timerfd = 5277,
    eventfd = 5278,
    fallocate = 5279,
    timerfd_create = 5280,
    timerfd_gettime = 5281,
    timerfd_settime = 5282,
    signalfd4 = 5283,
    eventfd2 = 5284,
    epoll_create1 = 5285,
    dup3 = 5286,
    pipe2 = 5287,
    inotify_init1 = 5288,
    preadv = 5289,
    pwritev = 5290,
    rt_tgsigqueueinfo = 5291,
    perf_event_open = 5292,
    accept4 = 5293,
    recvmmsg = 5294,
    fanotify_init = 5295,
    fanotify_mark = 5296,
    prlimit64 = 5297,
    name_to_handle_at = 5298,
    open_by_handle_at = 5299,
    clock_adjtime = 5300,
    syncfs = 5301,
    sendmmsg = 5302,
    setns = 5303,
    process_vm_readv = 5304,
    process_vm_writev = 5305,
    kcmp = 5306,
    finit_module = 5307,
    getdents64 = 5308,
    sched_setattr = 5309,
    sched_getattr = 5310,
    renameat2 = 5311,
    seccomp = 5312,
    getrandom = 5313,
    memfd_create = 5314,
    bpf = 5315,
    execveat = 5316,
    userfaultfd = 5317,
    membarrier = 5318,
    mlock2 = 5319,
    copy_file_range = 5320,
    preadv2 = 5321,
    pwritev2 = 5322,
    pkey_mprotect = 5323,
    pkey_alloc = 5324,
    pkey_free = 5325,
    statx = 5326,
    rseq = 5327,
    io_pgetevents = 5328,
    pidfd_send_signal = 5424,
    io_uring_setup = 5425,
    io_uring_enter = 5426,
    io_uring_register = 5427,
    open_tree = 5428,
    move_mount = 5429,
    fsopen = 5430,
    fsconfig = 5431,
    fsmount = 5432,
    fspick = 5433,
    pidfd_open = 5434,
    clone3 = 5435,
    close_range = 5436,
    openat2 = 5437,
    pidfd_getfd = 5438,
    faccessat2 = 5439,
    process_madvise = 5440,
    epoll_pwait2 = 5441,
    mount_setattr = 5442,
    quotactl_fd = 5443,
    landlock_create_ruleset = 5444,
    landlock_add_rule = 5445,
    landlock_restrict_self = 5446,
    process_mrelease = 5448,
    futex_waitv = 5449,
}
