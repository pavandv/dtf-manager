use const_format::formatcp;

pub const SSH_DIR: &str = "~/.ssh";

pub const OMZ_INSTALL_URL: &str =
    "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh";

pub const LOCAL_STARSHIP_PATH: &str = "~/.config/starship.toml";
pub const LOCAL_ZSHRC_PATH: &str = "~/.zshrc";

pub const DOT_FILES_PATH: &str = "~/.dotfiles";
pub const DTF_MANAGER_FILE: &str = ".dtf_manager";

pub const REMOTE_ZSHRC_PATH: &str = formatcp!("{dir}/.zshrc", dir = DOT_FILES_PATH);
// pub const REMOTE_STARSHIP_PATH: &str = formatcp!("{dir}/starship.toml", dir = DOT_FILES_PATH);

pub const HAMMERSPOON_CONFIG_DIR: &str = "~/.hammerspoon";
pub const HAMMERSPOON_CONFIG_FILE: &str = "init.lua";
pub const HAMMERSPOON_CONFIG_PATH: &str = formatcp!(
    "{dir}/{file}",
    dir = HAMMERSPOON_CONFIG_DIR,
    file = HAMMERSPOON_CONFIG_FILE
);
