use crate::config::Config;
use crate::utils::path_utils::PathUtils;
use crate::SgResult;
use std::cell::{RefCell, RefMut};

pub struct CommandContext {
    config: RefCell<Config>,
}

impl CommandContext {
    pub fn new() -> SgResult<Self> {
        let context = Self {
            config: Config::figment().extract()?,
        };

        Ok(context)
    }

    pub fn get_config(&self) -> RefMut<'_, Config> {
        self.config.borrow_mut()
    }

    pub fn is_dtf_managed(&self) -> bool {
        PathUtils::get_config_file_path().exists()
    }
}
