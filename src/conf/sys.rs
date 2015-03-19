use result::{Result};
use libc::{c_int, sysconf};
use errno::{self};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SysConf(pub c_int);

impl SysConf {
    pub fn get(self) -> Result<i64> {
        errno::set(0);
        let res = unsafe { sysconf(self.0) };
        if res == -1 {
            let err = errno::get();
            match err.0 {
                0 => Ok(res as i64),
                _ => Err(err),
            }
        } else {
            Ok(res as i64)
        }
    }
}

/// Maximum length of `exec` argument including environment data.
pub const ARG_MAX:                      SysConf = SysConf(0);
/// Maximum number of process per real user id.
pub const CHILD_MAX:                    SysConf = SysConf(1);
/// Clock ticks per second.
pub const CLK_TCK:                      SysConf = SysConf(2);
/// Maximum number of supplementary group ids.
pub const NGROUPS_MAX:                  SysConf = SysConf(3);
/// Maximum number of open file descriptors per process.
pub const OPEN_MAX:                     SysConf = SysConf(4);
/// Maximum number of open libc streams per process.
pub const STREAM_MAX:                   SysConf = SysConf(5);
/// Maximum length of a timezone name.
pub const TZNAME_MAX:                   SysConf = SysConf(6);
/// Support for job control.
pub const JOB_CONTROL:                  SysConf = SysConf(7);
/// Support for saved user and group ids.
pub const SAVED_IDS:                    SysConf = SysConf(8);
/// Support for realtime signals.
pub const REALTIME_SIGNALS:             SysConf = SysConf(9);
/// Support for priority scheduling.
pub const PRIORITY_SCHEDULING:          SysConf = SysConf(10);
/// Support for timers.
pub const TIMERS:                       SysConf = SysConf(11);
/// Support for asynchronous IO.
pub const ASYNCHRONOUS_IO:              SysConf = SysConf(12);
/// Support for prioritized IO.
pub const PRIORITIZED_IO:               SysConf = SysConf(13);
/// Support for synchronized IO.
pub const SYNCHRONIZED_IO:              SysConf = SysConf(14);
/// Support for file synchronization.
pub const FSYNC:                        SysConf = SysConf(15);
/// Support for memory mapped files.
pub const MAPPED_FILES:                 SysConf = SysConf(16);
/// Support for memory locking.
pub const MEMLOCK:                      SysConf = SysConf(17);
/// Support for ranged memory locking.
pub const MEMLOCK_RANGE:                SysConf = SysConf(18);
/// Support for memory protection.
pub const MEMORY_PROTECTION:            SysConf = SysConf(19);
/// Support for message queues.
pub const MESSAGE_PASSING:              SysConf = SysConf(20);
/// Support for semaphores.
pub const SEMAPHORES:                   SysConf = SysConf(21);
/// Support for shared memory objects.
pub const SHARED_MEMORY_OBJECTS:        SysConf = SysConf(22);
/// Maximum number of I/O operations in a single list I/O call.
pub const AIO_LISTIO_MAX:               SysConf = SysConf(23);
/// Maximum number of incomplete async I/O opreations.
pub const AIO_MAX:                      SysConf = SysConf(24);
/// Maximum number a process can decrease its async I/O priority from its scheduling
/// priority.
pub const AIO_PRIO_DELTA_MAX:           SysConf = SysConf(25);
/// Maximum number of timer expiration overruns.
pub const DELAYTIMER_MAX:               SysConf = SysConf(26);
/// Maximum number of open message queues per process.
pub const MQ_OPEN_MAX:                  SysConf = SysConf(27);
/// Maximum number of message queue priorities.
pub const MQ_PRIO_MAX:                  SysConf = SysConf(28);
/// POSIX version.
pub const VERSION:                      SysConf = SysConf(29);
/// Size of a memory page.
pub const PAGESIZE:                     SysConf = SysConf(30);
/// Number of reserved realtime signals.
pub const RTSIG_MAX:                    SysConf = SysConf(31);
/// Maximum number of semaphores per process.
pub const SEM_NSEMS_MAX:                SysConf = SysConf(32);
/// Maximum value of a sempahore.
pub const SEM_VALUE_MAX:                SysConf = SysConf(33);
/// Maximum number of queued signals per process.
pub const SIGQUEUE_MAX:                 SysConf = SysConf(34);
/// Maximum number of timers per process.
pub const TIMER_MAX:                    SysConf = SysConf(35);
/// Maximum output base of bc utility.
pub const BC_BASE_MAX:                  SysConf = SysConf(36);
/// Maximum number of elements per array in the bc utility.
pub const BC_DIM_MAX:                   SysConf = SysConf(37);
/// Maximum number of decimal digits maintained by the bc utility.
pub const BC_SCALE_MAX:                 SysConf = SysConf(38);
/// Maximum string length accepted by the bc utility.
pub const BC_STRING_MAX:                SysConf = SysConf(39);
/// Maximum number of weights per entry in the LC_COLLATE order keyword.
pub const COLL_WEIGHTS_MAX:             SysConf = SysConf(40);
/// Maximum number of weights per entry in the LC_COLLATE order keyword.
pub const EQUIV_CLASS_MAX:              SysConf = SysConf(41);
/// Maximum depth of nested expressions in the expr utility.
pub const EXPR_NEST_MAX:                SysConf = SysConf(42);
/// Maximum line length accepted by POSIX utilities.
pub const LINE_MAX:                     SysConf = SysConf(43);
/// Maximum number of repeated occurrences of a BRE or ERE interval expression
pub const RE_DUP_MAX:                   SysConf = SysConf(44);
/// Maximum length of a character class name.
pub const CHARCLASS_NAME_MAX:           SysConf = SysConf(45);
//pub const 2_VERSION:                    SysConf = SysConf(46);
//pub const 2_C_BIND:                     SysConf = SysConf(47);
//pub const 2_C_DEV:                      SysConf = SysConf(48);
//pub const 2_FORT_DEV:                   SysConf = SysConf(49);
//pub const 2_FORT_RUN:                   SysConf = SysConf(50);
//pub const 2_SW_DEV:                     SysConf = SysConf(51);
//pub const 2_LOCALEDEF:                  SysConf = SysConf(52);
pub const PII:                          SysConf = SysConf(53);
/// Support for XTI DNI
pub const PII_XTI:                      SysConf = SysConf(54);
/// Support for XTI sockets.
pub const PII_SOCKET:                   SysConf = SysConf(55);
/// Support for Internet Protocol
pub const PII_INTERNET:                 SysConf = SysConf(56);
/// Support for Open Systems Interconnection (OSI)
pub const PII_OSI:                      SysConf = SysConf(57);
/// Support for poll(2)
pub const POLL:                         SysConf = SysConf(58);
/// Support for select(2)
pub const SELECT:                       SysConf = SysConf(59);
pub const UIO_MAXIOV:                   SysConf = SysConf(60);
/// Maximum number of iovec structures per `readv` or `writev` call.
pub const IOV_MAX:                      SysConf = SysConf(60);
/// Support for TCP.
pub const PII_INTERNET_STREAM:          SysConf = SysConf(61);
/// Support for UDP.
pub const PII_INTERNET_DGRAM:           SysConf = SysConf(62);
pub const PII_OSI_COTS:                 SysConf = SysConf(63);
pub const PII_OSI_CLTS:                 SysConf = SysConf(64);
pub const PII_OSI_M:                    SysConf = SysConf(65);
pub const T_IOV_MAX:                    SysConf = SysConf(66);
/// Support for threads.
pub const THREADS:                      SysConf = SysConf(67);
/// Support for thread safe functions.
pub const THREAD_SAFE_FUNCTIONS:        SysConf = SysConf(68);
/// Maximum size of group info buffer.
pub const GETGR_R_SIZE_MAX:             SysConf = SysConf(69);
/// Maximum size of user info buffer.
pub const GETPW_R_SIZE_MAX:             SysConf = SysConf(70);
/// Maximum length of login name.
pub const LOGIN_NAME_MAX:               SysConf = SysConf(71);
/// Maximum length of tty device name.
pub const TTY_NAME_MAX:                 SysConf = SysConf(72);
/// Number of attempts made to destroy thread local data.
pub const THREAD_DESTRUCTOR_ITERATIONS: SysConf = SysConf(73);
/// Maximum number of pthread keys.
pub const THREAD_KEYS_MAX:              SysConf = SysConf(74);
/// Minimum stack size per thread.
pub const THREAD_STACK_MIN:             SysConf = SysConf(75);
/// Maximum number of threads per process.
pub const THREAD_THREADS_MAX:           SysConf = SysConf(76);
pub const THREAD_ATTR_STACKADDR:        SysConf = SysConf(77);
pub const THREAD_ATTR_STACKSIZE:        SysConf = SysConf(78);
pub const THREAD_PRIORITY_SCHEDULING:   SysConf = SysConf(79);
pub const THREAD_PRIO_INHERIT:          SysConf = SysConf(80);
pub const THREAD_PRIO_PROTECT:          SysConf = SysConf(81);
pub const THREAD_PROCESS_SHARED:        SysConf = SysConf(82);
pub const NPROCESSORS_CONF:             SysConf = SysConf(83);
pub const NPROCESSORS_ONLN:             SysConf = SysConf(84);
pub const PHYS_PAGES:                   SysConf = SysConf(85);
pub const AVPHYS_PAGES:                 SysConf = SysConf(86);
pub const ATEXIT_MAX:                   SysConf = SysConf(87);
pub const PASS_MAX:                     SysConf = SysConf(88);
pub const XOPEN_VERSION:                SysConf = SysConf(89);
pub const XOPEN_XCU_VERSION:            SysConf = SysConf(90);
pub const XOPEN_UNIX:                   SysConf = SysConf(91);
pub const XOPEN_CRYPT:                  SysConf = SysConf(92);
pub const XOPEN_ENH_I18N:               SysConf = SysConf(93);
pub const XOPEN_SHM:                    SysConf = SysConf(94);
//pub const 2_CHAR_TERM:                  SysConf = SysConf(95);
//pub const 2_C_VERSION:                  SysConf = SysConf(96);
//pub const 2_UPE:                        SysConf = SysConf(97);
pub const XOPEN_XPG2:                   SysConf = SysConf(98);
pub const XOPEN_XPG3:                   SysConf = SysConf(99);
pub const XOPEN_XPG4:                   SysConf = SysConf(100);
pub const CHAR_BIT:                     SysConf = SysConf(101);
pub const CHAR_MAX:                     SysConf = SysConf(102);
pub const CHAR_MIN:                     SysConf = SysConf(103);
pub const INT_MAX:                      SysConf = SysConf(104);
pub const INT_MIN:                      SysConf = SysConf(105);
pub const LONG_BIT:                     SysConf = SysConf(106);
pub const WORD_BIT:                     SysConf = SysConf(107);
pub const MB_LEN_MAX:                   SysConf = SysConf(108);
pub const NZERO:                        SysConf = SysConf(109);
pub const SSIZE_MAX:                    SysConf = SysConf(110);
pub const SCHAR_MAX:                    SysConf = SysConf(111);
pub const SCHAR_MIN:                    SysConf = SysConf(112);
pub const SHRT_MAX:                     SysConf = SysConf(113);
pub const SHRT_MIN:                     SysConf = SysConf(114);
pub const UCHAR_MAX:                    SysConf = SysConf(115);
pub const UINT_MAX:                     SysConf = SysConf(116);
pub const ULONG_MAX:                    SysConf = SysConf(117);
pub const USHRT_MAX:                    SysConf = SysConf(118);
pub const NL_ARGMAX:                    SysConf = SysConf(119);
pub const NL_LANGMAX:                   SysConf = SysConf(120);
pub const NL_MSGMAX:                    SysConf = SysConf(121);
pub const NL_NMAX:                      SysConf = SysConf(122);
pub const NL_SETMAX:                    SysConf = SysConf(123);
pub const NL_TEXTMAX:                   SysConf = SysConf(124);
pub const XBS5_ILP32_OFF32:             SysConf = SysConf(125);
pub const XBS5_ILP32_OFFBIG:            SysConf = SysConf(126);
pub const XBS5_LP64_OFF64:              SysConf = SysConf(127);
pub const XBS5_LPBIG_OFFBIG:            SysConf = SysConf(128);
pub const XOPEN_LEGACY:                 SysConf = SysConf(129);
pub const XOPEN_REALTIME:               SysConf = SysConf(130);
pub const XOPEN_REALTIME_THREADS:       SysConf = SysConf(131);
pub const ADVISORY_INFO:                SysConf = SysConf(132);
pub const BARRIERS:                     SysConf = SysConf(133);
pub const BASE:                         SysConf = SysConf(134);
pub const C_LANG_SUPPORT:               SysConf = SysConf(135);
pub const C_LANG_SUPPORT_R:             SysConf = SysConf(136);
pub const CLOCK_SELECTION:              SysConf = SysConf(137);
pub const CPUTIME:                      SysConf = SysConf(138);
pub const THREAD_CPUTIME:               SysConf = SysConf(139);
pub const DEVICE_IO:                    SysConf = SysConf(140);
pub const DEVICE_SPECIFIC:              SysConf = SysConf(141);
pub const DEVICE_SPECIFIC_R:            SysConf = SysConf(142);
pub const FD_MGMT:                      SysConf = SysConf(143);
pub const FIFO:                         SysConf = SysConf(144);
pub const PIPE:                         SysConf = SysConf(145);
pub const FILE_ATTRIBUTES:              SysConf = SysConf(146);
pub const FILE_LOCKING:                 SysConf = SysConf(147);
pub const FILE_SYSTEM:                  SysConf = SysConf(148);
pub const MONOTONIC_CLOCK:              SysConf = SysConf(149);
pub const MULTI_PROCESS:                SysConf = SysConf(150);
pub const SINGLE_PROCESS:               SysConf = SysConf(151);
pub const NETWORKING:                   SysConf = SysConf(152);
pub const READER_WRITER_LOCKS:          SysConf = SysConf(153);
pub const SPIN_LOCKS:                   SysConf = SysConf(154);
pub const REGEXP:                       SysConf = SysConf(155);
pub const REGEX_VERSION:                SysConf = SysConf(156);
pub const SHELL:                        SysConf = SysConf(157);
pub const SIGNALS:                      SysConf = SysConf(158);
pub const SPAWN:                        SysConf = SysConf(159);
pub const SPORADIC_SERVER:              SysConf = SysConf(160);
pub const THREAD_SPORADIC_SERVER:       SysConf = SysConf(161);
pub const SYSTEM_DATABASE:              SysConf = SysConf(162);
pub const SYSTEM_DATABASE_R:            SysConf = SysConf(163);
pub const TIMEOUTS:                     SysConf = SysConf(164);
pub const TYPED_MEMORY_OBJECTS:         SysConf = SysConf(165);
pub const USER_GROUPS:                  SysConf = SysConf(166);
pub const USER_GROUPS_R:                SysConf = SysConf(167);
//pub const 2_PBS:                        SysConf = SysConf(168);
//pub const 2_PBS_ACCOUNTING:             SysConf = SysConf(169);
//pub const 2_PBS_LOCATE:                 SysConf = SysConf(170);
//pub const 2_PBS_MESSAGE:                SysConf = SysConf(171);
//pub const 2_PBS_TRACK:                  SysConf = SysConf(172);
pub const SYMLOOP_MAX:                  SysConf = SysConf(173);
pub const STREAMS:                      SysConf = SysConf(174);
//pub const 2_PBS_CHECKPOINT:             SysConf = SysConf(175);
pub const V6_ILP32_OFF32:               SysConf = SysConf(176);
pub const V6_ILP32_OFFBIG:              SysConf = SysConf(177);
pub const V6_LP64_OFF64:                SysConf = SysConf(178);
pub const V6_LPBIG_OFFBIG:              SysConf = SysConf(179);
pub const HOST_NAME_MAX:                SysConf = SysConf(180);
pub const TRACE:                        SysConf = SysConf(181);
pub const TRACE_EVENT_FILTER:           SysConf = SysConf(182);
pub const TRACE_INHERIT:                SysConf = SysConf(183);
pub const TRACE_LOG:                    SysConf = SysConf(184);
/// Size of the level one instruction cache.
pub const LEVEL1_ICACHE_SIZE:           SysConf = SysConf(185);
/// Associativity of the level one instruction cache.
pub const LEVEL1_ICACHE_ASSOC:          SysConf = SysConf(186);
/// Line size of the level one instruction cache.
pub const LEVEL1_ICACHE_LINESIZE:       SysConf = SysConf(187);
/// Size of the level one data cache.
pub const LEVEL1_DCACHE_SIZE:           SysConf = SysConf(188);
/// Associativity of the level one data cache.
pub const LEVEL1_DCACHE_ASSOC:          SysConf = SysConf(189);
/// Line size of the level one data cache.
pub const LEVEL1_DCACHE_LINESIZE:       SysConf = SysConf(190);
/// Size of the level two cache.
pub const LEVEL2_CACHE_SIZE:            SysConf = SysConf(191);
/// Associativity of the level two cache.
pub const LEVEL2_CACHE_ASSOC:           SysConf = SysConf(192);
/// Line size of the level two cache.
pub const LEVEL2_CACHE_LINESIZE:        SysConf = SysConf(193);
/// Size of the level three cache.
pub const LEVEL3_CACHE_SIZE:            SysConf = SysConf(194);
/// Associativity of the level three cache.
pub const LEVEL3_CACHE_ASSOC:           SysConf = SysConf(195);
/// Line size of the level three cache.
pub const LEVEL3_CACHE_LINESIZE:        SysConf = SysConf(196);
/// Size of the level four cache.
pub const LEVEL4_CACHE_SIZE:            SysConf = SysConf(197);
/// Associativity of the level four cache.
pub const LEVEL4_CACHE_ASSOC:           SysConf = SysConf(198);
/// Line size of the level four cache.
pub const LEVEL4_CACHE_LINESIZE:        SysConf = SysConf(199);
pub const IPV6:                         SysConf = SysConf(235);
pub const RAW_SOCKETS:                  SysConf = SysConf(236);
pub const V7_ILP32_OFF32:               SysConf = SysConf(237);
pub const V7_ILP32_OFFBIG:              SysConf = SysConf(238);
pub const V7_LP64_OFF64:                SysConf = SysConf(239);
pub const V7_LPBIG_OFFBIG:              SysConf = SysConf(240);
pub const SS_REPL_MAX:                  SysConf = SysConf(241);
pub const TRACE_EVENT_NAME_MAX:         SysConf = SysConf(242);
pub const TRACE_NAME_MAX:               SysConf = SysConf(243);
pub const TRACE_SYS_MAX:                SysConf = SysConf(244);
pub const TRACE_USER_EVENT_MAX:         SysConf = SysConf(245);
pub const XOPEN_STREAMS:                SysConf = SysConf(246);
pub const THREAD_ROBUST_PRIO_INHERIT:   SysConf = SysConf(247);
pub const THREAD_ROBUST_PRIO_PROTECT:   SysConf = SysConf(248);
