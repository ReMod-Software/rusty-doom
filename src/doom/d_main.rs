extern crate libc;
use super::d_deh;
use super::m_argv;

static mut nomonsters: bool = false;
static mut fastparm: bool = false;
static mut respawnparm: bool = false;
static mut forceOldBsp: bool = false;

const standard_iwads: &[&str] = &[
    "doom2f.wad",
    "doom2.wad",
    "plutonia.wad",
    "tnt.wad",
    "doom.wad",
    "doom1.wad",
    "doomu.wad",
    "freedoom2.wad",
    "freedoom1.wad",
    "freedm.wad",
    "hacx.wad",
    "chex.wad",
    "rekkrsa.wad",
    "bfgdoom2.wad",
    "bfgdoom.wad",
];

unsafe fn D_DoomMainSetup() {
    let (mut p, mut slot) = (0, 0);

    let mut rsp_found = true;

    while rsp_found {
        rsp_found = false;
        for arg in m_argv::myargv() {
            let chr = arg.as_str().chars().nth(0).unwrap();
            if chr == '@' {
                println!("Found");
            }
        }
    }

    if m_argv::M_CheckParm("-forceoldbsp") > 0 {
        forceOldBsp = true;
    }

    d_deh::D_BuildBEXTables();
}

unsafe fn D_DoomLoop() {}

/*
 *  Main Function
 * */
pub fn D_DoomMain() {
    unsafe {
        D_DoomMainSetup();
        D_DoomLoop();
    }
}
