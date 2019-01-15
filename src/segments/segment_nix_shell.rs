use std::{env};
use {Powerline, Segment};

pub fn segment_nix_shell(p: &mut Powerline) {
    if let Ok(nix_shell) = env::var("IN_NIX_SHELL") {
        p.segments.push(Segment::new(
            p.theme.nix_shell_bg,
            p.theme.nix_shell_fg,
            nix_shell
        ));
    }
}
