// Copyright (c) 2024 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use api::{
    attestation_agent_service_client::AttestationAgentServiceClient,
    ExtendRuntimeMeasurementRequest, GetTokenRequest,
};
use clap::{Parser, Subcommand};
use log::info;

mod api {
    #![allow(unknown_lints)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    #![allow(clippy::redundant_async_block)]
    tonic::include_proto!("attestation_agent");
}

#[derive(Debug, Parser)]
#[command(author)]
struct Cli {
    /// Address of Attestation Agent
    ///
    /// `--address http://127.0.0.1:50000`
    #[arg(short, long, value_parser)]
    address: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Do attestation and get CoCo AS Token
    #[clap(arg_required_else_help = true)]
    Attestation {
        #[clap(long, value_parser)]
        token_type: String,
    },

    /// Record App level eventlog
    #[clap(arg_required_else_help = true)]
    RecordAppEventlog {
        #[clap(long, value_parser)]
        domain: String,

        #[clap(long, value_parser)]
        operation: String,

        #[clap(long, value_parser)]
        content: String,
    },
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let cli = Cli::parse();

    info!("Try to connect to AA: {}", cli.address);

    let mut client = AttestationAgentServiceClient::connect(cli.address)
        .await
        .unwrap();
    match cli.command {
        Commands::Attestation { token_type } => {
            let req = tonic::Request::new(GetTokenRequest { token_type });
            let token = client.get_token(req).await.unwrap().into_inner().token;
            let token = String::from_utf8(token).unwrap();
            println!("{token}");
        }
        Commands::RecordAppEventlog {
            domain,
            operation,
            content,
        } => {
            let req = tonic::Request::new(ExtendRuntimeMeasurementRequest {
                domain,
                operation,
                content,
                register_index: None,
            });
            client.extend_runtime_measurement(req).await.unwrap();
        }
    }
}
