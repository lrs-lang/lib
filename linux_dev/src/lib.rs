// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "linux_dev"]
#![crate_type = "lib"]

extern crate "linux_core" as core;

/// Device Id <-> Device Name mapping
///
/// Source: http://www.lanana.org/docs/device-list/devices-2.6+.txt

use std::io::{Write};

use core::cty::{dev_t};
use core::string::{LinuxString};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeviceType {
    Character,
    Block,
}

pub type DeviceId = dev_t;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Device(DeviceId, DeviceType);

impl Device {
    pub fn from_id(id: DeviceId, ty: DeviceType) -> Device {
        Device(id, ty)
    }

    pub fn from_major_minor(major: u32, minor: u32, ty: DeviceType) -> Device {
        let x = major as u64;
        let y = minor as u64;
        let id = (((x & 0xfffff000) << 32) | ((y & 0x00000fff) << 8) |
                  ((y & 0xffffff00) << 12) | ((y & 0x000000ff))) as DeviceId;
        Device(id, ty)
    }

    pub fn id(self) -> DeviceId {
        self.0
    }

    pub fn major(self) -> u32 {
        let x = self.0 as u64;
        (((x >> 32) & 0xfffff000) | ((x >> 8) & 0x00000fff)) as u32
    }

    pub fn minor(self) -> u32 {
        let x = self.0 as u64;
        (((x >> 12) & 0xffffff00) | (x & 0x000000ff)) as u32
    }

    pub fn to_path(self) -> LinuxString {
        path_from_device(self)
    }
}

fn path_from_device(d: Device) -> LinuxString {
    match d.1 {
        DeviceType::Character => path_from_char_device(d),
        DeviceType::Block     => path_from_block_device(d),
    }
}

fn path_from_char_device(d: Device) -> LinuxString {
    let mut base = b"/dev/".to_vec();
    macro_rules! p {
        ($fmt:expr) => {{ let _ = write!(base, $fmt); }};
        ($fmt:expr, $($var:tt)*) => {{ let _ = write!(base, $fmt, $($var)*); }}
    };
    macro_rules! invalid {
        () => { base = b"[Invalid]".to_vec() }
    };
    let major = d.major();
    let minor = d.minor() as u8;
    macro_rules! or_invalid {
        ($fmt:expr) => { if minor == 0 { p!($fmt) } else { invalid!() } }
    };
    match major {
        0 => base = b"[Unnamed]".to_vec(),
        1 => match minor {
            1  => p!("mem"),
            2  => p!("kmem"),
            3  => p!("null"),
            4  => p!("port"),
            5  => p!("zero"),
            6  => p!("core"),
            7  => p!("full"),
            8  => p!("random"),
            9  => p!("urandom"),
            10 => p!("aio"),
            11 => p!("kmsg"),
            _ => invalid!(),
        },
        2 => p!("pty{}{}", ptty_id_to_letter(minor), minor & 0xF),
        3 => p!("tty{}{}", ptty_id_to_letter(minor), minor & 0xF),
        4 => match minor {
            0 ... 63 => p!("tty{}", minor),
            _ => p!("ttyS{}", minor - 64),
        },
        5 => match minor {
            0 => p!("tty"),
            1 => p!("console"),
            2 => p!("ptmx"),
            3 ... 63 => invalid!(),
            _ => p!("cua{}", minor - 64),
        },
        6 => p!("lp{}", minor),
        7 => {
            if (minor > 63 && minor < 128) || minor > 191 {
                invalid!()
            } else {
                let letter = if minor < 128 { "" } else { "a" };
                if minor % 128 == 0 {
                    p!("vcs{}", letter)
                } else {
                    p!("vcs{}{}", letter, minor % 128)
                }
            }
        }
        9 => {
            let prefix = if minor < 128 { "" } else { "n" };
            let postfix = match (minor % 128) / 32 {
                0 => "",
                1 => "l",
                2 => "m",
                _ => "a",
            };
            p!("{}st{}{}", prefix, minor % 32, postfix)
        }
        10 => match minor {
            0   => p!("logibm"),
            1   => p!("psaux"),
            2   => p!("inportbm"),
            3   => p!("atibm"),
            4   => p!("jbm"),
            5   => p!("atarimouse"),
            6   => p!("sunmouse"),
            7   => p!("amigamouse1"),
            8   => p!("smouse"),
            9   => p!("pc110pad"),
            10  => p!("adbmouse"),
            11  => p!("vrtpanel"),
            13  => p!("vpcmouse"),
            14  => p!("touchscreenUcb1x00"),
            15  => p!("touchscreenMk712"),
            128 => p!("beep"),
            129 => p!("modreq"),
            130 => p!("watchdog"),
            131 => p!("temperature"),
            132 => p!("hwtrap"),
            133 => p!("exttrp"),
            134 => p!("apm_bios"),
            135 => p!("rtc"),
            139 => p!("openprom"),
            140 => p!("relay8"),
            141 => p!("relay16"),
            142 => p!("msr"),
            143 => p!("pciconf"),
            144 => p!("nvram"),
            145 => p!("hfmodem"),
            146 => p!("graphics"),
            147 => p!("opengl"),
            148 => p!("gfx"),
            149 => p!("inputMouse"),
            150 => p!("inputKeyboard"),
            151 => p!("led"),
            152 => p!("kpoll"),
            153 => p!("mergemem"),
            154 => p!("pmu"),
            155 => p!("isictl"),
            156 => p!("lcd"),
            157 => p!("ac"),
            158 => p!("nwbutton"),
            159 => p!("nwdebug"),
            160 => p!("nwflash"),
            161 => p!("userdma"),
            162 => p!("smbus"),
            163 => p!("lik"),
            164 => p!("ipmo"),
            165 => p!("vmmon"),
            166 => p!("i2oCtl"),
            167 => p!("specialix_sxctl"),
            168 => p!("tcldrv"),
            169 => p!("specialix_rioctl"),
            170 => p!("thinkpadThinkpad"),
            171 => p!("srripc"),
            172 => p!("usemaclone"),
            173 => p!("ipmikcs"),
            174 => p!("uctrl"),
            175 => p!("agpgart"),
            176 => p!("gtrsc"),
            177 => p!("cbm"),
            178 => p!("jsflash"),
            179 => p!("xsvc"),
            180 => p!("vrbuttons"),
            181 => p!("toshiba"),
            182 => p!("perfctr"),
            183 => p!("hwrng"),
            184 => p!("cpuMicrocode"),
            186 => p!("atomicps"),
            187 => p!("irnet"),
            188 => p!("smbusbios"),
            189 => p!("ussp_ctl"),
            190 => p!("crash"),
            191 => p!("pcl181"),
            192 => p!("nas_xbus"),
            193 => p!("d7s"),
            194 => p!("zkshim"),
            195 => p!("elographicsE2201"),
            198 => p!("sexec"),
            199 => p!("scannersCuecat"),
            200 => p!("netTun"),
            201 => p!("buttonGulpb"),
            202 => p!("emdCtl"),
            204 => p!("videoEm8300"),
            205 => p!("videoEm8300_mv"),
            206 => p!("videoEm8300_ma"),
            207 => p!("videoEm8300_sp"),
            208 => p!("compaqCpqphpc"),
            209 => p!("compaqCpqrid"),
            210 => p!("impiBt"),
            211 => p!("impiSmic"),
            212 => p!("watchdogs0"),
            213 => p!("watchdogs1"),
            214 => p!("watchdogs2"),
            215 => p!("watchdogs3"),
            216 => p!("fujitsuApanel"),
            217 => p!("niNatmotn"),
            218 => p!("kchuid"),
            219 => p!("modemsMwave"),
            220 => p!("mptctl"),
            221 => p!("mvistaHssdsi"),
            222 => p!("mvistaHasi"),
            223 => p!("inputUinput"),
            224 => p!("tpm"),
            225 => p!("pps"),
            226 => p!("systrace"),
            227 => p!("mcelog"),
            228 => p!("hpet"),
            229 => p!("fuse"),
            230 => p!("midishare"),
            _ => invalid!(),
        },
        11 => or_invalid!("kbd"),
        12 => match minor {
            2 => p!("ntpqic11"),
            3 => p!("tpqic11"),
            4 => p!("ntpqic24"),
            5 => p!("tpqic24"),
            6 => p!("ntpqic120"),
            7 => p!("tpqic120"),
            8 => p!("ntpqic150"),
            9 => p!("tpqic150"),
            _ => invalid!(),
        },
        13 => match (minor / 32, minor % 32) {
            (0, r)  => p!("input/js{}",    r),
            (1, 31) => p!("input/mice"),
            (1, r)  => p!("input/mouse{}", r),
            (2, r)  => p!("input/event{}", r),
            _ => invalid!(),
        },
        14 => match minor {
            0  => p!("mixer"),
            1  => p!("sequencer"),
            2  => p!("midi00"),
            3  => p!("dsp"),
            4  => p!("audio"),
            6  => p!("sndstat"),
            7  => p!("audioctl"),
            8  => p!("sequencer2"),
            16 => p!("mixer1"),
            17 => p!("patmgr0"),
            18 => p!("midi01"),
            19 => p!("dsp1"),
            20 => p!("audio1"),
            33 => p!("patmgr1"),
            34 => p!("midi02"),
            50 => p!("midi03"),
            _ => invalid!(),
        },
        15 => match minor / 128 {
            0 => p!("js{}", minor % 128),
            _ => p!("djs{}", minor % 128),
        },
        16 => or_invalid!("gs4500"),
        17 => p!("ttyH{}", minor),
        18 => p!("cuh{}",  minor),
        19 => p!("ttyC{}", minor),
        20 => p!("cub{}",  minor),
        21 => p!("sg{}",   minor),
        22 => p!("ttyD{}", minor),
        23 => p!("cud{}",  minor),
        24 => p!("ttyE{}", minor),
        25 => p!("cue{}",  minor),
        26 => or_invalid!("wvisfgrab"),
        27 => {
            let prefix = if minor % 8 > 3 { "n" } else { "" };
            match minor / 16 {
                0 => p!("{}qft{}",    prefix, minor % 4),
                1 => p!("{}zqft{}",   prefix, minor % 4),
                _ => p!("{}rawqft{}", prefix, minor % 4),
            }
        }
        28 => p!("staliomem{}", minor),
        29 => p!("fb{}", minor),
        30 => match minor {
            0  => p!("socksys"),
            1  => p!("spx"),
            32 => p!("inet/ip"),
            33 => p!("inet/icmp"),
            34 => p!("inet/ggp"),
            35 => p!("inet/ipip"),
            36 => p!("inet/tcp"),
            37 => p!("inet/egp"),
            38 => p!("inet/pup"),
            39 => p!("inet/udp"),
            40 => p!("inet/idp"),
            41 => p!("inet/rawip"),
            _  => invalid!(),
        },
        31 => match minor {
            0 => p!("mpu401data"),
            1 => p!("mpu401stat"),
            _ => invalid!(),
        },
        32 => p!("ttyX{}", minor),
        33 => p!("cux{}", minor),
        34 => p!("scc{}", minor),
        35 => match minor / 64 {
            0 => p!("midi{}",  minor % 64),
            1 => p!("rmidi{}", minor % 64),
            2 => p!("smpte{}", minor % 64),
            _ => invalid!(),
        },
        36 => match minor {
            0 => p!("route"),
            1 => p!("skip"),
            2 => p!("fwmonitor"),
            _ => p!("tap{}", minor - 16),
        },
        37 => match minor / 128 {
            0 => p!("ht{}",  minor % 128),
            _ => p!("nht{}", minor % 128),
        },
        38 => p!("mlanai{}", minor),
        39 => {
            let letter = (b'a' + (minor / 32)) as char;
            match minor % 32 {
                0 ... 15 => p!("ml16p{}-a{}", letter, minor % 32),
                16 => p!("ml16p{}-d", letter),
                17 ... 19 => p!("ml16p{}-c{}", letter, (minor % 32) - 17),
                _ => invalid!(),
            }
        }
        40 => or_invalid!("mmetfgrab"),
        41 => or_invalid!("yamm"),
        43 => p!("ttyI{}", minor),
        44 => p!("cui{}", minor),
        45 => {
            if minor == 255 {
                p!("isdninfo")
            } else {
                match minor / 64 {
                    0 => p!("isdn{}",     minor % 64),
                    1 => p!("isdnctrl{}", minor % 64),
                    2 => p!("ippp{}",     minor % 64),
                    _ => invalid!(),
                }
            }
        }
        46 => p!("ttyR{}",  minor),
        47 => p!("cur{}",   minor),
        48 => p!("ttyL{}",  minor),
        49 => p!("cul{}",   minor),
        51 => p!("bc{}",    minor),
        52 => p!("dcbri{}", minor),
        53 => match minor {
            0 ... 2 => p!("pd_bdm{}", minor),
            4 ... 5 => p!("icd_bdm{}", minor - 4),
            _ => invalid!(),
        },
        54 => p!("holter{}", minor),
        55 => or_invalid!("dsp56k"),
        56 => or_invalid!("adb"),
        57 => p!("ttyP{}", minor),
        58 => p!("cup{}", minor),
        59 => or_invalid!("firewall"),
        64 => or_invalid!("enskip"),
        65 => match minor / 64 {
            0 => p!("plink{}",   minor % 64),
            1 => p!("rplink{}",  minor % 64),
            2 => p!("plink{}d",  minor % 64),
            _ => p!("rplink{}d", minor % 64),
        },
        66 => p!("yppcpci{}", minor),
        67 => p!("cfs0"),
        68 => match minor {
            0 => p!("capi20"),
            _ => p!("capi20.{}", minor)
        },
        69 => or_invalid!("ma16"),
        70 => match minor {
            0   => p!("apscfg"),
            1   => p!("apsauth"),
            2   => p!("apslog"),
            3   => p!("apsdbg"),
            64  => p!("apsisdn"),
            65  => p!("apsasync"),
            128 => p!("apsmon"),
            _ => invalid!(),
        },
        71 => p!("ttyF{}", minor),
        72 => p!("cuf{}", minor),
        73 => {
            let infix = if minor % 2 == 0 { "ipl" } else { "stat" };
            p!("ip2{}{}", infix, minor / 4)
        }
        74 => p!("SCI/{}", minor),
        75 => p!("ttyW{}", minor),
        76 => p!("cuw{}", minor),
        77 => or_invalid!("qng"),
        78 => p!("ttyM{}", minor),
        79 => p!("cum{}", minor),
        80 => or_invalid!("at200"),
        81 => match minor {
            0 ... 63 => p!("video{}", minor),
            64 ... 127 => p!("radio{}", minor - 64),
            192 ... 223 => p!("vtx{}", minor - 192),
            224 ... 255 => p!("vbi{}", minor - 224),
            _ => invalid!(),
        },
        82 => p!("winradio{}", minor),
        83 => p!("mga_vid{}", minor),
        84 => p!("ihcp{}", minor),
        85 => match minor {
            0 => p!("shimq"),
            _ => p!("qcntl{}", minor),
        },
        86 => p!("sch{}", minor),
        87 => p!("controla{}", minor),
        88 => p!("comx{}", minor),
        89 => p!("i2c-{}", minor),
        90 => match minor % 2 {
            0 => p!("mtd{}", minor / 2),
            _ => p!("mtdr{}", minor / 2),
        },
        91 => p!("can{}", minor),
        93 => match minor / 128 {
            0 => p!("iscc{}", minor),
            _ => p!("isccctl{}", minor - 128),
        },
        94 => p!("dcxx{}", minor),
        95 => match minor {
            0 => p!("ipl"),
            1 => p!("ipnat"),
            2 => p!("ipstate"),
            3 => p!("ipauth"),
            _ => invalid!(),
        },
        96 => match minor / 128 {
            0 => p!("pt{}", minor),
            _ => p!("npt{}", minor - 128),
        },
        97 => p!("pg{}", minor),
        98 => p!("comedi{}", minor),
        99 => p!("paraport{}", minor),
        100 => p!("phone{}", minor),
        101 => match minor {
            0 => p!("mdspstat"),
            _ => p!("mdsp{}", minor),
        },
        102 => p!("tlk{}", minor),
        103 => p!("nnpfs{}", minor),
        105 => p!("ttyV{}", minor),
        106 => p!("cuv{}", minor),
        107 => or_invalid!("3dfx"),
        108 => or_invalid!("ppp"),
        110 => p!("srnd{}", minor),
        111 => p!("av{}", minor),
        112 => p!("ttyM{}", minor),
        113 => p!("cum{}", minor),
        114 => match minor / 128 {
            0 => p!("ise{}", minor),
            _ => p!("isex{}", minor - 128),
        },
        115 => match minor / 8 {
            0 => p!("tipar{}", minor),
            1 => p!("tiser{}", minor - 8),
            _ => p!("tiusb{}", minor - 16),
        },
        117 => p!("cosa{}c{}", minor / 16, minor % 16),
        118 => match minor {
            0 => p!("ica"),
            _ => p!("ica{}", minor - 1),
        },
        119 => p!("vnet{}", minor),
        136 ... 143 => p!("pts/{}", minor),
        144 => p!("pppox{}", minor),
        145 => match minor % 64 {
            0  => p!("sam{}_mixer",     minor / 64),
            1  => p!("sam{}_sequencer", minor / 64),
            2  => p!("sam{}_midi00",    minor / 64),
            3  => p!("sam{}_dsp",       minor / 64),
            4  => p!("sam{}_audio",     minor / 64),
            6  => p!("sam{}_sndstat",   minor / 64),
            18 => p!("sam{}_midi01",    minor / 64),
            34 => p!("sam{}_midi02",    minor / 64),
            50 => p!("sam{}_midi03",    minor / 64),
            _ => invalid!(),
        },
        146 => p!("scramnet{}", minor),
        147 => p!("aureal{}", minor),
        148 => p!("ttyT{}", minor),
        149 => p!("cut{}", minor),
        150 => p!("rtf{}", minor),
        151 => p!("dpti{}", minor),
        152 => match minor {
            0 => p!("etherd/ctl"),
            1 => p!("etherd/err"),
            2 => p!("etherd/raw"),
            _ => invalid!(),
        },
        153 => p!("spi{}", minor),
        154 => p!("ttySR{}", minor),
        155 => p!("cusr{}", minor),
        156 => p!("ttySR{}", minor as u32 + 256),
        157 => p!("cusr{}", minor as u32 + 256),
        158 => p!("gfax{}", minor),
        160 => p!("gpib{}", minor),
        161 => match minor / 16 {
            0 => p!("ircomm{}", minor),
            _ => p!("irlpt{}", minor - 16),
        },
        162 => match minor {
            0 => p!("rawctl"),
            _ => p!("raw/raw{}", minor),
        },
        164 => match minor {
            0 ... 63 => p!("ttyCH{}", minor),
            _ => invalid!(),
        },
        165 => match minor {
            0 ... 63 => p!("cuch{}", minor),
            _ => invalid!(),
        },
        166 => p!("ttyACM{}", minor),
        167 => p!("cuacm{}", minor),
        168 => p!("ecsa{}", minor),
        169 => p!("ecsa8-{}", minor),
        170 => p!("megarac{}", minor),
        172 => match minor {
            0 ... 127 => p!("ttyMX{}", minor),
            128 => p!("moxactl"),
            _ => invalid!(),
        },
        173 => match minor {
            0 ... 127 => p!("cumx{}", minor),
            _ => invalid!(),
        },
        174 => p!("ttySI{}", minor),
        175 => p!("cusi{}", minor),
        176 => p!("nfastpci{}", minor),
        177 => match minor / 16 {
            0 => p!("pcilynx/aux{}", minor % 16),
            1 => p!("pcilynx/rom{}", minor % 16),
            2 => p!("pcilynx/ram{}", minor % 16),
            _ => invalid!(),
        },
        178 => p!("clanvi{}", minor),
        179 => p!("dvxirq{}", minor),
        180 => match minor / 16 {
            0 => p!("usb/lp{}", minor % 16),
            1 => p!("usb/mouse{}", minor % 16),
            2 => p!("usb/ez{}", minor % 16),
            3 => p!("usb/scanner{}", minor % 16),
            _ => match minor {
                64 => p!("usb/rio500"),
                65 => p!("usb/usblcd"),
                66 => p!("usb/cpad0"),
                _ => invalid!(),
            },
        },
        181 => p!("pcfclock{}", minor),
        182 => p!("pethr{}", minor),
        183 => p!("ss5036dn{}", minor),
        184 => p!("pevss{}", minor),
        185 => p!("intermezzo{}", minor),
        186 => p!("obd{}", minor),
        187 => p!("deskey{}", minor),
        188 => p!("ttyUSB{}", minor),
        189 => p!("cuusb{}", minor),
        190 => p!("kctt{}", minor),
        192 => match minor {
            0 => p!("profile"),
            _ => p!("profile{}", minor - 1),
        },
        193 => match minor {
            0 => p!("trace"),
            _ => p!("trace{}", minor - 1),
        },
        194 => match minor % 16 {
            0 => p!("mvideo/status{}",      minor / 16),
            1 => p!("mvideo/stream{}",      minor / 16),
            2 => p!("mvideo/frame{}",       minor / 16),
            3 => p!("mvideo/rawframe{}",    minor / 16),
            4 => p!("mvideo/codec{}",       minor / 16),
            5 => p!("mvideo/video4linux{}", minor / 16),
            _ => invalid!(),
        },
        195 => match minor {
            255 => p!("nvidiactl"),
            _ => p!("nvidia{}", minor),
        },
        196 => p!("tor/{}", minor),
        197 => match minor {
            0 ... 127 => p!("tnf/t{}", minor),
            128 => p!("tnf/status"),
            130 => p!("tnf/trace"),
            _ => invalid!(),
        },
        198 => p!("tpmp2/{}", minor),
        200 => match minor {
            0 => p!("vx/config"),
            1 => p!("vx/trace"),
            2 => p!("vx/iod"),
            3 => p!("vx/info"),
            4 => p!("vx/task"),
            5 => p!("vx/taskmon"),
            _ => invalid!(),
        },
        202 => p!("cpu/{}/msr", minor),
        203 => p!("cpu/{}/cpuid", minor),
        204 => match minor {
            0  => p!("ttyLU0"),
            1  => p!("ttyLU1"),
            2  => p!("ttyLU2"),
            3  => p!("ttyLU3"),
            4  => p!("ttyFB0"),
            5  => p!("ttySA0"),
            6  => p!("ttySA1"),
            7  => p!("ttySA2"),
            8  => p!("ttySC0"),
            9  => p!("ttySC1"),
            10 => p!("ttySC2"),
            11 => p!("ttySC3"),
            12 => p!("ttyFW0"),
            13 => p!("ttyFW1"),
            14 => p!("ttyFW2"),
            15 => p!("ttyFW3"),
            16 ... 31 => p!("ttyAM{}", minor - 16),
            32 ... 39 => p!("ttyDB{}", minor - 32),
            40 => p!("ttySG0"),
            41 => p!("ttySMX0"),
            42 => p!("ttySMX1"),
            43 => p!("ttySMX2"),
            44 => p!("ttyMM0"),
            45 => p!("ttyMM1"),
            46 ... 49 => p!("ttyCPM{}", minor - 46), // the documentation isn't clear here
            50 ... 81 => p!("ttyIOC4{}", minor - 50),
            _ => invalid!(),
        },
        205 => match minor {
            0  => p!("culu0"),
            1  => p!("culu1"),
            2  => p!("culu2"),
            3  => p!("culu3"),
            4  => p!("cufb0"),
            5  => p!("cusa0"),
            6  => p!("cusa1"),
            7  => p!("cusa2"),
            8  => p!("cusc0"),
            9  => p!("cusc1"),
            10 => p!("cusc2"),
            11 => p!("cusc3"),
            12 => p!("cufw0"),
            13 => p!("cufw1"),
            14 => p!("cufw2"),
            15 => p!("cufw3"),
            16 ... 31 => p!("cuam{}", minor - 16),
            32 ... 39 => p!("cudb{}", minor - 32),
            40 => p!("cusg0"),
            41 => p!("cusmX0"),
            42 => p!("cusmX1"),
            43 => p!("cusmX2"),
            44 => p!("cumm0"),
            45 => p!("cumm1"),
            46 ... 49 => p!("cucpm{}", minor - 46), // the documentation isn't clear here
            50 ... 81 => p!("cuioc4{}", minor - 50),
            _ => invalid!(),
        },
        206 => match minor / 32 {
            0 => p!("osst{}", minor % 32),
            1 => p!("osst{}l", minor % 32),
            2 => p!("osst{}m", minor % 32),
            3 => p!("osst{}a", minor % 32),
            4 => p!("nosst{}", minor % 32),
            5 => p!("nosst{}l", minor % 32),
            6 => p!("nosst{}m", minor % 32),
            _ => p!("nosst{}a", minor % 32),
        },
        207 => match minor {
            0  => p!("cpqhealth/cpqw"),
            1  => p!("cpqhealth/crom"),
            2  => p!("cpqhealth/cdt"),
            3  => p!("cpqhealth/cevt"),
            4  => p!("cpqhealth/casr"),
            5  => p!("cpqhealth/cecc"),
            6  => p!("cpqhealth/cmca"),
            7  => p!("cpqhealth/ccsm"),
            8  => p!("cpqhealth/cnmi"),
            9  => p!("cpqhealth/css"),
            10 => p!("cpqhealth/cram"),
            11 => p!("cpqhealth/cpci"),
            _ => invalid!(),
        },
        208 => p!("ttyU{}", minor),
        209 => p!("cuu{}", minor),
        210 => match minor % 10 { // Yes, decimal 10
            0 => p!("sbei/wxcfg{}", minor / 10),
            1 => p!("sbei/dld{}",   minor / 10),
            2 => p!("sbei/wan{}0",  minor / 10),
            3 => p!("sbei/wan{}1",  minor / 10),
            4 => p!("sbei/wan{}2",  minor / 10),
            5 => p!("sbei/wan{}3",  minor / 10),
            6 => p!("sbei/wanc{}0", minor / 10),
            7 => p!("sbei/wanc{}1", minor / 10),
            8 => p!("sbei/wanc{}2", minor / 10),
            _ => p!("sbei/wanc{}3", minor / 10),
        },
        211 => p!("addinum/cpci1500/{}", minor),
        212 => match (minor % 64) % 9 { // Documentation unclear
            0 => p!("dvb/adapter{}/video{}",    minor / 64, (minor % 64) / 9),
            1 => p!("dvb/adapter{}/audio{}",    minor / 64, (minor % 64) / 9),
            2 => p!("dvb/adapter{}/sec{}",      minor / 64, (minor % 64) / 9),
            3 => p!("dvb/adapter{}/frontend{}", minor / 64, (minor % 64) / 9),
            4 => p!("dvb/adapter{}/demux{}",    minor / 64, (minor % 64) / 9),
            5 => p!("dvb/adapter{}/dvr{}",      minor / 64, (minor % 64) / 9),
            6 => p!("dvb/adapter{}/ca{}",       minor / 64, (minor % 64) / 9),
            7 => p!("dvb/adapter{}/net{}",      minor / 64, (minor % 64) / 9),
            8 => p!("dvb/adapter{}/osd{}",      minor / 64, (minor % 64) / 9),
            _ => invalid!(),
        },
        216 => p!("ttyUB{}", minor),
        217 => p!("cuub{}", minor),
        218 => p!("logicalco/bci/{}", minor),
        219 => p!("logicalco/dci1300/{}", minor),
        220 => match minor % 2 {
            0 => p!("myricom/gm{}",  minor / 2),
            _ => p!("myricom/gmp{}", minor / 2),
        },
        221 => match minor {
		    0 => p!("bus/vme/m0"),
		    1 => p!("bus/vme/m1"),
		    2 => p!("bus/vme/m2"),
		    3 => p!("bus/vme/m3"),
		    4 => p!("bus/vme/s0"),
		    5 => p!("bus/vme/s1"),
		    6 => p!("bus/vme/s2"),
		    7 => p!("bus/vme/s3"),
		    8 => p!("bus/vme/ctl"),
            _ => invalid!(),
        },
        224 => p!("ttyY{}", minor),
        225 => p!("cuy{}", minor),
        226 => p!("dri/card{}", minor),
        227 => match minor {
            0 => invalid!(),
            _ => p!("3270/tty{}", minor),
        },
        228 => match minor {
            0 => p!("3270/tub"),
            _ => p!("3270/tub{}", minor),
        },
        229 => p!("iseries/vtty{}", minor),
        230 => match minor / 32 {
            0 => p!("iseries/vt{}",   minor % 32),
            1 => p!("iseries/vt{}l",  minor % 32),
            2 => p!("iseries/vt{}m",  minor % 32),
            3 => p!("iseries/vt{}a",  minor % 32),
            4 => p!("iseries/nvt{}",  minor % 32),
            5 => p!("iseries/nvt{}l", minor % 32),
            6 => p!("iseries/nvt{}m", minor % 32),
            7 => p!("iseries/nvt{}a", minor % 32),
            _ => invalid!(),
        },
        231 => p!("infiniband/umad{}", minor),
        _ => invalid!(),
    }
    LinuxString::from_vec(base)
}

fn path_from_block_device(d: Device) -> LinuxString {
    let mut base = b"/dev/".to_vec();
    macro_rules! p {
        ($fmt:expr) => {{ let _ = write!(base, $fmt); }};
        ($fmt:expr, $($var:tt)*) => {{ let _ = write!(base, $fmt, $($var)*); }}
    };
    macro_rules! invalid {
        () => { base = b"[Invalid]".to_vec() }
    };
    let major = d.major();
    let minor = d.minor() as u8;
    macro_rules! or_invalid {
        ($fmt:expr) => { if minor == 0 { p!($fmt) } else { invalid!() } }
    };
    match major {
        0 => base = b"[Unnamed]".to_vec(),
        1 => p!("ram{}", minor),
        2 => {
            let base = if minor < 128 { 0 } else { 4 };
            let rest = minor % 128;
            p!("fd");
            p!("{}", base + minor % 4);
            match rest / 4 {
                0  => p!(""),
                1  => p!("d360"),
                2  => p!("h1200"),
                3  => p!("u360"),
                4  => p!("u720"),
                5  => p!("h360"),
                6  => p!("h720"),
                7  => p!("u1440"),
                8  => p!("u2880"),
                9  => p!("CompaQ"),
                10 => p!("h1440"),
                11 => p!("u1680"),
                12 => p!("h410"),
                13 => p!("u820"),
                14 => p!("h1476"),
                15 => p!("u1722"),
                16 => p!("h420"),
                17 => p!("u830"),
                18 => p!("h1494"),
                19 => p!("u1743"),
                20 => p!("h880"),
                21 => p!("u1040"),
                22 => p!("u1120"),
                23 => p!("h1600"),
                24 => p!("u1760"),
                25 => p!("u1920"),
                26 => p!("u3200"),
                27 => p!("u3520"),
                28 => p!("u3840"),
                29 => p!("u1840"),
                30 => p!("u800"),
                _  => p!("u1600"),
            }
        },
        3 => {
            let letter = (b'a' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("hd{}", letter),
                n => p!("hd{}{}", letter, n),
            }
        },
        4 => p!("root"),
        7 => p!("loop{}", minor),
        8 => {
            let letter = (b'a' + (minor / 16)) as char;
            match minor % 16 {
                0 => p!("sd{}", letter),
                n => p!("sd{}{}", letter, n),
            }
        },
        9 => p!("md{}", minor),
        11 => p!("scd{}", minor),
        12 => p!("dos_cd{}", minor),
        13 => {
            let letter = (b'a' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("xd{}", letter),
                n => p!("xd{}{}", letter, n),
            }
        },
        14 => {
            let letter = (b'a' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("dos_hd{}", letter),
                n => p!("dos_hd{}{}", letter, n),
            }
        },
        15 => or_invalid!("sonycd"),
        16 => or_invalid!("gscd"),
        17 => or_invalid!("optcd"),
        18 => or_invalid!("sjcd"),
        19 => match minor / 128 {
            0 => p!("double{}", minor % 128),
            _ => p!("cdouble{}", minor % 128),
        },
        20 => or_invalid!("hitcd"),
        21 => {
            let letter = (b'a' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("mfm{}", letter),
                n => p!("mfm{}{}", letter, n),
            }
        },
        22 => {
            let letter = (b'c' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("hd{}", letter),
                n => p!("hd{}{}", letter, n),
            }
        },
        23 => or_invalid!("mcd"),
        24 => or_invalid!("cdu535"),
        25 ... 28 => match minor {
            0 ... 4 => p!("sbpcd{}", minor as u32 + major - 25),
            _ => invalid!(),
        },
        29 => or_invalid!("aztcd"),
        30 => or_invalid!("cm205cd"),
        31 => match minor / 8 {
            0 => p!("rom{}", minor % 8),
            1 => p!("rrom{}", minor % 8),
            2 => p!("flash{}", minor % 8),
            3 => p!("rflash{}", minor % 8),
            _ => invalid!(),
        },
        32 => or_invalid!("cm206cd"),
        33 => {
            let letter = (b'e' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("hd{}", letter),
                n => p!("hd{}{}", letter, n),
            }
        },
        34 => {
            let letter = (b'g' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("hd{}", letter),
                n => p!("hd{}{}", letter, n),
            }
        },
        35 => or_invalid!("slram"),
        36 => {
            let letter = (b'a' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("ed{}", letter),
                n => p!("ed{}{}", letter, n),
            }
        },
        37 => or_invalid!("z2ram"),
        40 => match minor {
            0 => p!("eza"),
            1 ... 63 => p!("eza{}", minor),
            _ => invalid!(),
        },
        41 => or_invalid!("bpcd"),
        43 => p!("nb{}", minor),
        44 => {
            let letter = (b'a' + (minor / 16)) as char;
            match minor % 16 {
                0 => p!("ftl{}", letter),
                n => p!("ftl{}{}", letter, n),
            }
        },
        45 => {
            let letter = (b'a' + (minor / 16)) as char;
            match minor % 16 {
                0 => p!("pd{}", letter),
                n => p!("pd{}{}", letter, n),
            }
        },
        46 => p!("pcd{}", minor),
        47 => p!("pf{}", minor),
        48 ... 55 => match minor % 8 {
            0 => p!("rd/c{}d{}",    major - 48, minor / 8),
            n => p!("rd/c{}d{}p{}", major - 48, minor / 8,  n),
        },
        56 => {
            let letter = (b'i' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("hd{}", letter),
                n => p!("hd{}{}", letter, n),
            }
        },
        57 => {
            let letter = (b'k' + (minor / 64)) as char;
            match minor % 64 {
                0 => p!("hd{}", letter),
                n => p!("hd{}{}", letter, n),
            }
        },
        59 => p!("pda{}", minor),
        64 => match minor {
            0 => p!("scramdisk/master"),
            _ => p!("scramdisk/{}", minor)
        },
        65 ... 71 => {
            // start of the 16th sd disk
            let disk_num = ((major as u32 - 65) * 256 + minor as u32) / 16 + 16;
            let len = (b'z' - b'a' + 1) as u32;
            // The first block doesn't have a first letter.
            let first_letter  = ((disk_num / len) as u8 + b'a' - 1) as char;
            let second_letter = ((disk_num % len) as u8 + b'a') as char;
            if first_letter >= 'a' {
                match minor % 16 {
                    0 => p!("sd{}{}",   first_letter, second_letter),
                    n => p!("sd{}{}{}", first_letter, second_letter, n),
                }
            } else {
                match minor % 16 {
                    0 => p!("sd{}",   second_letter),
                    n => p!("sd{}{}", second_letter, n),
                }
            }
        },
        72 ... 79 => match minor % 16 {
            0 => p!("ida/c{}d{}",    major - 72, minor / 16),
            n => p!("ida/c{}d{}p{}", major - 72, minor / 16,  n),
        },
        80 ... 87 => {
            let disk_num = ((major as u32 - 80) * 256 + minor as u32) / 16;
            let len = (b'z' - b'a' + 1) as u32;
            // The first block doesn't have a first letter.
            let first_letter  = ((disk_num / len) as u8 + b'a' - 1) as char;
            let second_letter = ((disk_num % len) as u8 + b'a') as char;
            if first_letter >= 'a' {
                match minor % 16 {
                    0 => p!("i2o/hd{}{}",   first_letter, second_letter),
                    n => p!("i2o/hd{}{}{}", first_letter, second_letter, n),
                }
            } else {
                match minor % 16 {
                    0 => p!("i2o/hd{}",   second_letter),
                    n => p!("i2o/hd{}{}", second_letter, n),
                }
            }
        },
        88 ... 91 => {
            let letter = (b'm' + (((major as u32 - 88) * 256 + minor as u32) / 64) as u8) as char;
            match minor % 64 {
                0 => p!("hd{}", letter),
                n => p!("hd{}{}", letter, n),
            }
        },
        92 => p!("ppdd{}", minor), // Documentation wrong
        93 => {
            let letter = (b'a' + (minor / 16)) as char;
            match minor % 16 {
                0 => p!("nftl{}", letter),
                n => p!("nftl{}{}", letter, n),
            }
        },
        94 => {
            let letter = (b'a' + (minor / 4)) as char;
            match minor % 4 {
                0 => p!("dasd{}", letter),
                n => p!("dasd{}{}", letter, n),
            }
        },
        96 => {
            let letter = (b'a' + (minor / 16)) as char;
            match minor % 16 {
                0 => p!("inftl{}", letter),
                n => p!("inftl{}{}", letter, n),
            }
        },
        97 => p!("pktcdvd{}", minor),
        98 => {
            let letter = (b'a' + (minor / 16)) as char;
            match minor % 16 {
                0 => p!("ubd{}", letter),
                n => p!("ubd{}{}", letter, n),
            }
        },
        99 => or_invalid!("jsfd"),
        101 => {
            match minor % 16 {
                0 => p!("amiraid/ar{}", minor / 16),
                n => p!("amiraid/ar{}p{}", minor / 16, n),
            }
        },
        102 => {
            let letter = (b'a' + (minor / 16)) as char;
            match minor % 16 {
                0 => p!("cbd/{}", letter),
                n => p!("cbd/{}{}", letter, n),
            }
        },
        103 => or_invalid!("audit"),
        104 ... 111 => match minor % 16 {
            0 => p!("cciss/c{}d{}",    major - 104, minor / 16),
            n => p!("cciss/c{}d{}p{}", major - 104, minor / 16,  n),
        },
        112 => {
            let disk_num = minor / 8;
            let len = b'z' - b'a' + 1;
            // The first block doesn't have a first letter.
            let first_letter  = (disk_num / len + b'a' - 1)    as char;
            let second_letter = (disk_num % len + b'a') as char;
            if first_letter >= 'a' {
                match minor % 8 {
                    0 => p!("iseries/vd{}{}",   first_letter, second_letter),
                    n => p!("iseries/vd{}{}{}", first_letter, second_letter, n),
                }
            } else {
                match minor % 8 {
                    0 => p!("iseries/vd{}",   second_letter),
                    n => p!("iseries/vd{}{}", second_letter, n),
                }
            }
        },
        113 => {
            let len = b'z' - b'a' + 1;
            // The first block doesn't have a first letter.
            let first_letter  = (minor / len + b'a' - 1)    as char;
            let second_letter = (minor % len + b'a') as char;
            if first_letter >= 'a' {
                p!("iseries/vcd{}{}", first_letter, second_letter)
            } else {
                p!("iseries/vcd{}", second_letter)
            }
        },
        114 => match minor % 16 {
            0 => p!("ataraid/d{}",  minor / 16),
            n => p!("ataraid/d{}p{}", minor / 16,  n),
        },
        115 => p!("nwfs/v{}", minor),
        116 => match minor % 16 {
            0 => p!("umem/d{}",  minor / 16),
            n => p!("umem/d{}p{}", minor / 16,  n),
        },
        128 ... 135 => {
            // start of the 128th sd disk
            let disk_num = ((major as u32 - 128) * 256 + minor as u32) / 16 + 128;
            let len = (b'z' - b'a' + 1) as u32;
            // The first block doesn't have a first letter.
            let first_letter  = ((disk_num / len) as u8 + b'a' - 1) as char;
            let second_letter = ((disk_num % len) as u8 + b'a') as char;
            if first_letter >= 'a' {
                match minor % 16 {
                    0 => p!("sd{}{}",   first_letter, second_letter),
                    n => p!("sd{}{}{}", first_letter, second_letter, n),
                }
            } else {
                match minor % 16 {
                    0 => p!("sd{}",   second_letter),
                    n => p!("sd{}{}", second_letter, n),
                }
            }
        },
        136 ... 143 => match minor % 8 {
            0 => p!("rd/c{}d{}",    major - 136 + 8, minor / 8),
            n => p!("rd/c{}d{}p{}", major - 136 + 8, minor / 8,  n),
        },
        147 => p!("drbd{}", minor),
        152 => p!("etherd/{}", minor),
        153 => match minor % 16 {
            0 => p!("emd/{}",  minor / 16),
            n => p!("emd/{}p{}", minor / 16,  n),
        },
        160 => match minor % 32 {
            0 => p!("carmel/{}",  minor / 32),
            n => p!("carmel/{}p{}", minor / 32,  n),
        },
        161 => match minor % 32 {
            0 => p!("carmel/{}",    minor / 32 + 8),
            n => p!("carmel/{}p{}", minor / 32 + 8,  n),
        },
        180 => match minor % 8 {
            0 => p!("ub{}",   minor / 8 + b'a'),
            n => p!("ub{}{}", minor / 8 + b'a', n),
        },
        _ => invalid!(),
    }
    LinuxString::from_vec(base)
}

fn ptty_id_to_letter(n: u8) -> char {
    let su = n >> 4;
    if su <= b'z' - b'p' {
        (b'p' + su) as char
    } else {
        (b'a' + su - b'z' + b'p') as char
    }
}
