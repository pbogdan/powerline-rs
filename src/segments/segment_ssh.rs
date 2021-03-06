use std::env;
use {Powerline, Segment};

pub fn segment_ssh(p: &mut Powerline) {
    if env::var("SSH_CLIENT").is_ok() {
        p.segments.push(Segment::new(p.theme.ssh_bg, p.theme.ssh_fg, ""));
    }
}
