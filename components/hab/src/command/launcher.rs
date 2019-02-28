// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ffi::OsString;

use crate::common::ui::UI;

use crate::error::Result;

pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> { inner::start(ui, args) }

#[cfg(not(target_os = "macos"))]
mod inner {
    use std::{ffi::OsString,
              path::PathBuf,
              str::FromStr};

    use crate::{common::ui::UI,
                hcore::{crypto::{default_cache_key_path,
                                 init},
                        env as henv,
                        fs::find_command,
                        os::process,
                        package::PackageIdent}};

    use super::super::sup::{SUP_CMD,
                            SUP_CMD_ENVVAR,
                            SUP_PKG_IDENT};
    use crate::{error::{Error,
                        Result},
                exec,
                VERSION};

    const LAUNCH_CMD: &str = "hab-launch";
    const LAUNCH_CMD_ENVVAR: &str = "HAB_LAUNCH_BINARY";
    const LAUNCH_PKG_IDENT: &str = "core/hab-launcher";

    pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        init();
        if henv::var(SUP_CMD_ENVVAR).is_err() {
            let version: Vec<&str> = VERSION.split('/').collect();
            exec::command_from_min_pkg(ui,
                                       SUP_CMD,
                                       &PackageIdent::from_str(&format!("{}/{}",
                                                                        SUP_PKG_IDENT,
                                                                        version[0]))?,
                                       &default_cache_key_path(None),
                                       0)?;
        }
        let command = match henv::var(LAUNCH_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                exec::command_from_min_pkg(ui,
                                           LAUNCH_CMD,
                                           &PackageIdent::from_str(LAUNCH_PKG_IDENT)?,
                                           &default_cache_key_path(None),
                                           0)?
            }
        };
        if let Some(cmd) = find_command(&command) {
            process::become_command(cmd, args)?;
            Ok(())
        } else {
            Err(Error::ExecCommandNotFound(command))
        }
    }
}

#[cfg(target_os = "macos")]
mod inner {
    use std::{env,
              ffi::OsString};

    use crate::common::ui::{UIWriter,
                            UI};

    use crate::error::{Error,
                       Result};

    pub fn start(ui: &mut UI, _args: Vec<OsString>) -> Result<()> {
        let subcmd = env::args().nth(1).unwrap_or("<unknown>".to_string());
        ui.warn("Launching a native Supervisor on this operating system is not yet supported. \
                 Try running this command again on 64-bit Linux or Windows.")?;
        ui.br()?;
        Err(Error::SubcommandNotSupported(subcmd))
    }
}
