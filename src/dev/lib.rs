// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![crate_name = "lrs_dev"]
#![crate_type = "lib"]
#![feature(plugin, no_std)]
#![plugin(lrs_core_plugin)]
#![no_std]
#![allow(trivial_numeric_casts)]

extern crate lrs_base as base;
extern crate lrs_fmt as fmt;
extern crate lrs_cty as cty;

use base::prelude::*;
mod std { pub use fmt::std::*; }

// Device Id <-> Device Name mapping
//
// Source: http://www.lanana.org/docs/device-list/devices-2.6+.txt

use fmt::{Write, Debug};
use cty::alias::{DeviceId};

/// The type of a device.
#[derive(Copy, Eq)]
pub enum DeviceType {
    /// A character device.
    Character,
    /// a block device.
    Block,
}

impl Debug for DeviceType {
    fn fmt<W: Write>(&self, w: &mut W) -> Result {
        (match *self {
            DeviceType::Character => w.write_all(b"Character"),
            DeviceType::Block => w.write_all(b"Block"),
        }).ignore_ok()
    }
}

/// A device.
///
/// [field, 1]
/// The id of the device.
///
/// [field, 2]
/// The type of the device.
#[derive(Copy, Eq)]
pub struct Device(DeviceId, DeviceType);

impl Device {
    /// Returns a device based on its id and type.
    ///
    /// [argument, id]
    /// The id of the device.
    ///
    /// [argument, ty]
    /// The type of the device.
    pub fn from_id(id: DeviceId, ty: DeviceType) -> Device {
        Device(id, ty)
    }

    /// Returns a device based on its major and minor numbers and type.
    ///
    /// [argument, major]
    /// The major part of the device id.
    ///
    /// [argument, minor]
    /// The minor part of the device id.
    ///
    /// [argument, ty]
    /// The type of the device.
    pub fn from_major_minor(major: u32, minor: u32, ty: DeviceType) -> Device {
        let x = major as u64;
        let y = minor as u64;
        let id = (((x & 0xfffff000) << 32) | ((y & 0x00000fff) << 8) |
                  ((y & 0xffffff00) << 12) | ((y & 0x000000ff))) as DeviceId;
        Device(id, ty)
    }

    /// Returns the device id of a device.
    pub fn id(self) -> DeviceId {
        self.0
    }

    /// Returns the major of a device.
    pub fn major(self) -> u32 {
        let x = self.0 as u64;
        (((x >> 32) & 0xfffff000) | ((x >> 8) & 0x00000fff)) as u32
    }

    /// Returns the type of a device.
    pub fn ty(self) -> DeviceType {
        self.1
    }

    /// Returns the minor of a device.
    pub fn minor(self) -> u32 {
        let x = self.0 as u64;
        (((x >> 12) & 0xffffff00) | (x & 0x000000ff)) as u32
    }

    /// Returns the path of a device in "/dev".
    ///
    /// [argument, buf]
    /// The buffer in which the device path will be stored.
    ///
    /// = Remarks
    ///
    /// This functionality is only available if lrs has been compiled with the
    /// `device_paths` option. Otherwise the function returns the `NotImplemented` error.
    ///
    /// = Example
    ///
    /// ----
    /// let device = Device::from_major_minor(8, 1, DeviceType::Block);
    /// assert_eq!(device.to_path(), "/dev/sda1");
    /// ----
    pub fn to_path(self, buf: &mut [u8]) -> Result<&mut [u8]> {
        dfmt::to_path(self, buf)
    }
}

#[cfg(not(device_paths))]
mod dfmt {
    use base::prelude::*;
    use super::{Device};
    use fmt::{Write, Debug};
    use base::{error};

    pub fn to_path(_: Device, _: &mut [u8]) -> Result<&mut [u8]> {
        Err(error::NotImplemented)
    }

    impl Debug for Device {
        fn fmt<W: Write>(&self, mut w: &mut W) -> Result {
            write!(w, "({}, {:?})", self.0, self.1)
        }
    }
}

#[cfg(device_paths)]
mod dfmt {
    #[prelude_import] use base::prelude::*;
    use super::{Device};
    use fmt::{Write, Debug};

    pub fn to_path(d: Device, buf: &mut [u8]) -> Result<&mut [u8]> {
        path_from_device(self, buf)
    }

    impl Debug for Device {
        fn fmt<W: Write>(&self, w: &mut W) -> Result {
            let mut buf = [0; 128];
            Debug::fmt(path_from_device_int(*self, &mut buf).as_byte_str(), w)
        }
    }

    fn path_from_device(d: Device, user_buf: &mut [u8]) -> Result<&mut [u8]> {
        let mut buf = [0; 128];
        let res = path_from_device_int(d, &mut buf);
        if user_buf.len() < res.len() {
            Err(error::NoMemory)
        } else {
            mem::copy(user_buf, res);
            Ok(&mut user_buf[..res.len()])
        }
    }

    fn path_from_device_int(d: Device, buf: &mut [u8]) -> &[u8] {
        let res = match d.1 {
            DeviceType::Character => path_from_char_device(d, buf),
            DeviceType::Block     => path_from_block_device(d, buf),
        };
        match res {
            0 => &b"[invalid]"[..],
            1 => &b"[unnamed]"[..],
            n => &buf[..n],
        }
    }

    fn path_from_char_device(d: Device, mut buf: &mut [u8]) -> usize {
        let old_len = buf.len();
        buf.write(b"/dev/");
        macro_rules! p {
            ($str:expr) => {{ buf.write($str); }};
            ($fmt:expr, $($var:tt)*) => {{ write!(buf, $fmt, $($var)*); }}
        };
        macro_rules! invalid {
            () => { return 0; }
        };
        let major = d.major();
        let minor = d.minor() as u8;
        macro_rules! or_invalid {
            ($str:expr) => { if minor == 0 { p!($str) } else { invalid!() } }
        };
        match major {
            0 => return 1,
            1 => match minor {
                1  => p!(b"mem"),
                2  => p!(b"kmem"),
                3  => p!(b"null"),
                4  => p!(b"port"),
                5  => p!(b"zero"),
                6  => p!(b"core"),
                7  => p!(b"full"),
                8  => p!(b"random"),
                9  => p!(b"urandom"),
                10 => p!(b"aio"),
                11 => p!(b"kmsg"),
                _ => invalid!(),
            },
            2 => p!("pty{}{}", ptty_id_to_letter(minor), minor & 0xF),
            3 => p!("tty{}{}", ptty_id_to_letter(minor), minor & 0xF),
            4 => match minor {
                0 ... 63 => p!("tty{}", minor),
                _ => p!("ttyS{}", minor - 64),
            },
            5 => match minor {
                0 => p!(b"tty"),
                1 => p!(b"console"),
                2 => p!(b"ptmx"),
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
                0   => p!(b"logibm"),
                1   => p!(b"psaux"),
                2   => p!(b"inportbm"),
                3   => p!(b"atibm"),
                4   => p!(b"jbm"),
                5   => p!(b"atarimouse"),
                6   => p!(b"sunmouse"),
                7   => p!(b"amigamouse1"),
                8   => p!(b"smouse"),
                9   => p!(b"pc110pad"),
                10  => p!(b"adbmouse"),
                11  => p!(b"vrtpanel"),
                13  => p!(b"vpcmouse"),
                14  => p!(b"touchscreenUcb1x00"),
                15  => p!(b"touchscreenMk712"),
                128 => p!(b"beep"),
                129 => p!(b"modreq"),
                130 => p!(b"watchdog"),
                131 => p!(b"temperature"),
                132 => p!(b"hwtrap"),
                133 => p!(b"exttrp"),
                134 => p!(b"apm_bios"),
                135 => p!(b"rtc"),
                139 => p!(b"openprom"),
                140 => p!(b"relay8"),
                141 => p!(b"relay16"),
                142 => p!(b"msr"),
                143 => p!(b"pciconf"),
                144 => p!(b"nvram"),
                145 => p!(b"hfmodem"),
                146 => p!(b"graphics"),
                147 => p!(b"opengl"),
                148 => p!(b"gfx"),
                149 => p!(b"inputMouse"),
                150 => p!(b"inputKeyboard"),
                151 => p!(b"led"),
                152 => p!(b"kpoll"),
                153 => p!(b"mergemem"),
                154 => p!(b"pmu"),
                155 => p!(b"isictl"),
                156 => p!(b"lcd"),
                157 => p!(b"ac"),
                158 => p!(b"nwbutton"),
                159 => p!(b"nwdebug"),
                160 => p!(b"nwflash"),
                161 => p!(b"userdma"),
                162 => p!(b"smbus"),
                163 => p!(b"lik"),
                164 => p!(b"ipmo"),
                165 => p!(b"vmmon"),
                166 => p!(b"i2oCtl"),
                167 => p!(b"specialix_sxctl"),
                168 => p!(b"tcldrv"),
                169 => p!(b"specialix_rioctl"),
                170 => p!(b"thinkpadThinkpad"),
                171 => p!(b"srripc"),
                172 => p!(b"usemaclone"),
                173 => p!(b"ipmikcs"),
                174 => p!(b"uctrl"),
                175 => p!(b"agpgart"),
                176 => p!(b"gtrsc"),
                177 => p!(b"cbm"),
                178 => p!(b"jsflash"),
                179 => p!(b"xsvc"),
                180 => p!(b"vrbuttons"),
                181 => p!(b"toshiba"),
                182 => p!(b"perfctr"),
                183 => p!(b"hwrng"),
                184 => p!(b"cpuMicrocode"),
                186 => p!(b"atomicps"),
                187 => p!(b"irnet"),
                188 => p!(b"smbusbios"),
                189 => p!(b"ussp_ctl"),
                190 => p!(b"crash"),
                191 => p!(b"pcl181"),
                192 => p!(b"nas_xbus"),
                193 => p!(b"d7s"),
                194 => p!(b"zkshim"),
                195 => p!(b"elographicsE2201"),
                198 => p!(b"sexec"),
                199 => p!(b"scannersCuecat"),
                200 => p!(b"netTun"),
                201 => p!(b"buttonGulpb"),
                202 => p!(b"emdCtl"),
                204 => p!(b"videoEm8300"),
                205 => p!(b"videoEm8300_mv"),
                206 => p!(b"videoEm8300_ma"),
                207 => p!(b"videoEm8300_sp"),
                208 => p!(b"compaqCpqphpc"),
                209 => p!(b"compaqCpqrid"),
                210 => p!(b"impiBt"),
                211 => p!(b"impiSmic"),
                212 => p!(b"watchdogs0"),
                213 => p!(b"watchdogs1"),
                214 => p!(b"watchdogs2"),
                215 => p!(b"watchdogs3"),
                216 => p!(b"fujitsuApanel"),
                217 => p!(b"niNatmotn"),
                218 => p!(b"kchuid"),
                219 => p!(b"modemsMwave"),
                220 => p!(b"mptctl"),
                221 => p!(b"mvistaHssdsi"),
                222 => p!(b"mvistaHasi"),
                223 => p!(b"inputUinput"),
                224 => p!(b"tpm"),
                225 => p!(b"pps"),
                226 => p!(b"systrace"),
                227 => p!(b"mcelog"),
                228 => p!(b"hpet"),
                229 => p!(b"fuse"),
                230 => p!(b"midishare"),
                _ => invalid!(),
            },
            11 => or_invalid!(b"kbd"),
            12 => match minor {
                2 => p!(b"ntpqic11"),
                3 => p!(b"tpqic11"),
                4 => p!(b"ntpqic24"),
                5 => p!(b"tpqic24"),
                6 => p!(b"ntpqic120"),
                7 => p!(b"tpqic120"),
                8 => p!(b"ntpqic150"),
                9 => p!(b"tpqic150"),
                _ => invalid!(),
            },
            13 => match (minor / 32, minor % 32) {
                (0, r)  => p!("input/js{}",    r),
                (1, 31) => p!(b"input/mice"),
                (1, r)  => p!("input/mouse{}", r),
                (2, r)  => p!("input/event{}", r),
                _ => invalid!(),
            },
            14 => match minor {
                0  => p!(b"mixer"),
                1  => p!(b"sequencer"),
                2  => p!(b"midi00"),
                3  => p!(b"dsp"),
                4  => p!(b"audio"),
                6  => p!(b"sndstat"),
                7  => p!(b"audioctl"),
                8  => p!(b"sequencer2"),
                16 => p!(b"mixer1"),
                17 => p!(b"patmgr0"),
                18 => p!(b"midi01"),
                19 => p!(b"dsp1"),
                20 => p!(b"audio1"),
                33 => p!(b"patmgr1"),
                34 => p!(b"midi02"),
                50 => p!(b"midi03"),
                _ => invalid!(),
            },
            15 => match minor / 128 {
                0 => p!("js{}", minor % 128),
                _ => p!("djs{}", minor % 128),
            },
            16 => or_invalid!(b"gs4500"),
            17 => p!("ttyH{}", minor),
            18 => p!("cuh{}",  minor),
            19 => p!("ttyC{}", minor),
            20 => p!("cub{}",  minor),
            21 => p!("sg{}",   minor),
            22 => p!("ttyD{}", minor),
            23 => p!("cud{}",  minor),
            24 => p!("ttyE{}", minor),
            25 => p!("cue{}",  minor),
            26 => or_invalid!(b"wvisfgrab"),
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
                0  => p!(b"socksys"),
                1  => p!(b"spx"),
                32 => p!(b"inet/ip"),
                33 => p!(b"inet/icmp"),
                34 => p!(b"inet/ggp"),
                35 => p!(b"inet/ipip"),
                36 => p!(b"inet/tcp"),
                37 => p!(b"inet/egp"),
                38 => p!(b"inet/pup"),
                39 => p!(b"inet/udp"),
                40 => p!(b"inet/idp"),
                41 => p!(b"inet/rawip"),
                _  => invalid!(),
            },
            31 => match minor {
                0 => p!(b"mpu401data"),
                1 => p!(b"mpu401stat"),
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
                0 => p!(b"route"),
                1 => p!(b"skip"),
                2 => p!(b"fwmonitor"),
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
            40 => or_invalid!(b"mmetfgrab"),
            41 => or_invalid!(b"yamm"),
            43 => p!("ttyI{}", minor),
            44 => p!("cui{}", minor),
            45 => {
                if minor == 255 {
                    p!(b"isdninfo")
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
            55 => or_invalid!(b"dsp56k"),
            56 => or_invalid!(b"adb"),
            57 => p!("ttyP{}", minor),
            58 => p!("cup{}", minor),
            59 => or_invalid!(b"firewall"),
            64 => or_invalid!(b"enskip"),
            65 => match minor / 64 {
                0 => p!("plink{}",   minor % 64),
                1 => p!("rplink{}",  minor % 64),
                2 => p!("plink{}d",  minor % 64),
                _ => p!("rplink{}d", minor % 64),
            },
            66 => p!("yppcpci{}", minor),
            67 => p!(b"cfs0"),
            68 => match minor {
                0 => p!(b"capi20"),
                _ => p!("capi20.{}", minor)
            },
            69 => or_invalid!(b"ma16"),
            70 => match minor {
                0   => p!(b"apscfg"),
                1   => p!(b"apsauth"),
                2   => p!(b"apslog"),
                3   => p!(b"apsdbg"),
                64  => p!(b"apsisdn"),
                65  => p!(b"apsasync"),
                128 => p!(b"apsmon"),
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
            77 => or_invalid!(b"qng"),
            78 => p!("ttyM{}", minor),
            79 => p!("cum{}", minor),
            80 => or_invalid!(b"at200"),
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
                0 => p!(b"shimq"),
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
                0 => p!(b"ipl"),
                1 => p!(b"ipnat"),
                2 => p!(b"ipstate"),
                3 => p!(b"ipauth"),
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
                0 => p!(b"mdspstat"),
                _ => p!("mdsp{}", minor),
            },
            102 => p!("tlk{}", minor),
            103 => p!("nnpfs{}", minor),
            105 => p!("ttyV{}", minor),
            106 => p!("cuv{}", minor),
            107 => or_invalid!(b"3dfx"),
            108 => or_invalid!(b"ppp"),
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
                0 => p!(b"ica"),
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
                0 => p!(b"etherd/ctl"),
                1 => p!(b"etherd/err"),
                2 => p!(b"etherd/raw"),
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
                0 => p!(b"rawctl"),
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
                128 => p!(b"moxactl"),
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
                    64 => p!(b"usb/rio500"),
                    65 => p!(b"usb/usblcd"),
                    66 => p!(b"usb/cpad0"),
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
                0 => p!(b"profile"),
                _ => p!("profile{}", minor - 1),
            },
            193 => match minor {
                0 => p!(b"trace"),
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
                255 => p!(b"nvidiactl"),
                _ => p!("nvidia{}", minor),
            },
            196 => p!("tor/{}", minor),
            197 => match minor {
                0 ... 127 => p!("tnf/t{}", minor),
                128 => p!(b"tnf/status"),
                130 => p!(b"tnf/trace"),
                _ => invalid!(),
            },
            198 => p!("tpmp2/{}", minor),
            200 => match minor {
                0 => p!(b"vx/config"),
                1 => p!(b"vx/trace"),
                2 => p!(b"vx/iod"),
                3 => p!(b"vx/info"),
                4 => p!(b"vx/task"),
                5 => p!(b"vx/taskmon"),
                _ => invalid!(),
            },
            202 => p!("cpu/{}/msr", minor),
            203 => p!("cpu/{}/cpuid", minor),
            204 => match minor {
                0  => p!(b"ttyLU0"),
                1  => p!(b"ttyLU1"),
                2  => p!(b"ttyLU2"),
                3  => p!(b"ttyLU3"),
                4  => p!(b"ttyFB0"),
                5  => p!(b"ttySA0"),
                6  => p!(b"ttySA1"),
                7  => p!(b"ttySA2"),
                8  => p!(b"ttySC0"),
                9  => p!(b"ttySC1"),
                10 => p!(b"ttySC2"),
                11 => p!(b"ttySC3"),
                12 => p!(b"ttyFW0"),
                13 => p!(b"ttyFW1"),
                14 => p!(b"ttyFW2"),
                15 => p!(b"ttyFW3"),
                16 ... 31 => p!("ttyAM{}", minor - 16),
                32 ... 39 => p!("ttyDB{}", minor - 32),
                40 => p!(b"ttySG0"),
                41 => p!(b"ttySMX0"),
                42 => p!(b"ttySMX1"),
                43 => p!(b"ttySMX2"),
                44 => p!(b"ttyMM0"),
                45 => p!(b"ttyMM1"),
                46 ... 49 => p!("ttyCPM{}", minor - 46), // the documentation isn't clear here
                50 ... 81 => p!("ttyIOC4{}", minor - 50),
                _ => invalid!(),
            },
            205 => match minor {
                0  => p!(b"culu0"),
                1  => p!(b"culu1"),
                2  => p!(b"culu2"),
                3  => p!(b"culu3"),
                4  => p!(b"cufb0"),
                5  => p!(b"cusa0"),
                6  => p!(b"cusa1"),
                7  => p!(b"cusa2"),
                8  => p!(b"cusc0"),
                9  => p!(b"cusc1"),
                10 => p!(b"cusc2"),
                11 => p!(b"cusc3"),
                12 => p!(b"cufw0"),
                13 => p!(b"cufw1"),
                14 => p!(b"cufw2"),
                15 => p!(b"cufw3"),
                16 ... 31 => p!("cuam{}", minor - 16),
                32 ... 39 => p!("cudb{}", minor - 32),
                40 => p!(b"cusg0"),
                41 => p!(b"cusmX0"),
                42 => p!(b"cusmX1"),
                43 => p!(b"cusmX2"),
                44 => p!(b"cumm0"),
                45 => p!(b"cumm1"),
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
                0  => p!(b"cpqhealth/cpqw"),
                1  => p!(b"cpqhealth/crom"),
                2  => p!(b"cpqhealth/cdt"),
                3  => p!(b"cpqhealth/cevt"),
                4  => p!(b"cpqhealth/casr"),
                5  => p!(b"cpqhealth/cecc"),
                6  => p!(b"cpqhealth/cmca"),
                7  => p!(b"cpqhealth/ccsm"),
                8  => p!(b"cpqhealth/cnmi"),
                9  => p!(b"cpqhealth/css"),
                10 => p!(b"cpqhealth/cram"),
                11 => p!(b"cpqhealth/cpci"),
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
                0 => p!(b"bus/vme/m0"),
                1 => p!(b"bus/vme/m1"),
                2 => p!(b"bus/vme/m2"),
                3 => p!(b"bus/vme/m3"),
                4 => p!(b"bus/vme/s0"),
                5 => p!(b"bus/vme/s1"),
                6 => p!(b"bus/vme/s2"),
                7 => p!(b"bus/vme/s3"),
                8 => p!(b"bus/vme/ctl"),
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
                0 => p!(b"3270/tub"),
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
        old_len - buf.len()
    }

    fn path_from_block_device(d: Device, mut buf: &mut [u8]) -> usize {
        let old_len = buf.len();
        buf.write(b"/dev/");
        macro_rules! p {
            ($str:expr) => {{ buf.write($str); }};
            ($fmt:expr, $($var:tt)*) => {{ write!(buf, $fmt, $($var)*); }}
        };
        macro_rules! invalid {
            () => { return 0; }
        };
        let major = d.major();
        let minor = d.minor() as u8;
        macro_rules! or_invalid {
            ($str:expr) => { if minor == 0 { p!($str) } else { invalid!() } }
        };
        match major {
            0 => return 1,
            1 => p!("ram{}", minor),
            2 => {
                let base = if minor < 128 { 0 } else { 4 };
                let rest = minor % 128;
                p!(b"fd");
                p!("{}", base + minor % 4);
                match rest / 4 {
                    0  => p!(b""),
                    1  => p!(b"d360"),
                    2  => p!(b"h1200"),
                    3  => p!(b"u360"),
                    4  => p!(b"u720"),
                    5  => p!(b"h360"),
                    6  => p!(b"h720"),
                    7  => p!(b"u1440"),
                    8  => p!(b"u2880"),
                    9  => p!(b"CompaQ"),
                    10 => p!(b"h1440"),
                    11 => p!(b"u1680"),
                    12 => p!(b"h410"),
                    13 => p!(b"u820"),
                    14 => p!(b"h1476"),
                    15 => p!(b"u1722"),
                    16 => p!(b"h420"),
                    17 => p!(b"u830"),
                    18 => p!(b"h1494"),
                    19 => p!(b"u1743"),
                    20 => p!(b"h880"),
                    21 => p!(b"u1040"),
                    22 => p!(b"u1120"),
                    23 => p!(b"h1600"),
                    24 => p!(b"u1760"),
                    25 => p!(b"u1920"),
                    26 => p!(b"u3200"),
                    27 => p!(b"u3520"),
                    28 => p!(b"u3840"),
                    29 => p!(b"u1840"),
                    30 => p!(b"u800"),
                    _  => p!(b"u1600"),
                }
            },
            3 => {
                let letter = (b'a' + (minor / 64)) as char;
                match minor % 64 {
                    0 => p!("hd{}", letter),
                    n => p!("hd{}{}", letter, n),
                }
            },
            4 => p!(b"root"),
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
            15 => or_invalid!(b"sonycd"),
            16 => or_invalid!(b"gscd"),
            17 => or_invalid!(b"optcd"),
            18 => or_invalid!(b"sjcd"),
            19 => match minor / 128 {
                0 => p!("double{}", minor % 128),
                _ => p!("cdouble{}", minor % 128),
            },
            20 => or_invalid!(b"hitcd"),
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
            23 => or_invalid!(b"mcd"),
            24 => or_invalid!(b"cdu535"),
            25 ... 28 => match minor {
                0 ... 4 => p!("sbpcd{}", minor as u32 + major - 25),
                _ => invalid!(),
            },
            29 => or_invalid!(b"aztcd"),
            30 => or_invalid!(b"cm205cd"),
            31 => match minor / 8 {
                0 => p!("rom{}", minor % 8),
                1 => p!("rrom{}", minor % 8),
                2 => p!("flash{}", minor % 8),
                3 => p!("rflash{}", minor % 8),
                _ => invalid!(),
            },
            32 => or_invalid!(b"cm206cd"),
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
            35 => or_invalid!(b"slram"),
            36 => {
                let letter = (b'a' + (minor / 64)) as char;
                match minor % 64 {
                    0 => p!("ed{}", letter),
                    n => p!("ed{}{}", letter, n),
                }
            },
            37 => or_invalid!(b"z2ram"),
            40 => match minor {
                0 => p!(b"eza"),
                1 ... 63 => p!("eza{}", minor),
                _ => invalid!(),
            },
            41 => or_invalid!(b"bpcd"),
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
                0 => p!(b"scramdisk/master"),
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
            99 => or_invalid!(b"jsfd"),
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
            103 => or_invalid!(b"audit"),
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
            254 => p!("dm-{}", minor), // not in the documentation
            _ => invalid!(),
        }
        old_len - buf.len()
    }

    fn ptty_id_to_letter(n: u8) -> char {
        let su = n >> 4;
        if su <= b'z' - b'p' {
            (b'p' + su) as char
        } else {
            (b'a' + su - b'z' + b'p') as char
        }
    }
}
