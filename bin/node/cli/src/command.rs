// Copyright 2020 Keysians Technologies.
// This file is part of Keysians.

// Keysians is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Keysians is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Keysians.  If not, see <http://www.gnu.org/licenses/>.

use crate::{chain_spec, service, Cli, Subcommand};
use node_runtime::{Block, RuntimeApi};
use sc_cli::{Result, SubstrateCli};

impl SubstrateCli for Cli {
	fn impl_name() -> &'static str {
		"keysians"
	}

	fn impl_version() -> &'static str {
		env!("CARGO_PKG_VERSION")
	}

	fn description() -> &'static str {
		env!("CARGO_PKG_DESCRIPTION")
	}

	fn author() -> &'static str {
		env!("CARGO_PKG_AUTHORS")
	}

	fn support_url() -> &'static str {
		"https://github.com/Keysians/keysians/issues/new"
	}

	fn copyright_start_year() -> i32 {
		2019
	}

	fn executable_name() -> &'static str {
		"keysians"
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::development_config()),
			"local" => Box::new(chain_spec::local_testnet_config()),
			"" | "fir" | "flaming-fir" => Box::new(chain_spec::flaming_fir_config()?),
			"staging" => Box::new(chain_spec::staging_testnet_config()),
			"keysians" => Box::new(chain_spec::keysians_testnet_config()),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node(
				service::new_light,
				service::new_full,
				node_runtime::VERSION
			)
		}
		Some(Subcommand::Inspect(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| cmd.run::<Block, RuntimeApi, crate::executor::Executor>(config))
		}
		Some(Subcommand::Base(subcommand)) => {
			let runner = cli.create_runner(subcommand)?;

			runner.run_subcommand(subcommand, |config| Ok(new_full_start!(config).0))
		}
	}
}