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

//! Keysians chain configurations.

use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use node_runtime::constants::currency::*;
use node_runtime::Block;
use node_runtime::{
    wasm_binary_unwrap, AssetsConfig, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig,
    BridgeEosConfig, ContractsConfig, ConvertConfig, CouncilConfig, DemocracyConfig,
    ElectionsConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig, SessionKeys,
    SocietyConfig, StakerStatus, StakingConfig, SudoConfig, SwapConfig, SystemConfig,
    TechnicalCommitteeConfig, VoucherConfig,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};

pub use node_primitives::{AccountAsset, AccountId, Balance, Cost, Income, Signature, TokenSymbol};
pub use node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
}

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
    // stash, controller, session-key
    // generated with secret:
    // for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
    // and
    // for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            // 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
            hex!["9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"].into(),
            // 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
            hex!["781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"].into(),
            // 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
            hex!["9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
        ),
        (
            // 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
            hex!["68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"].into(),
            // 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
            hex!["c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"].into(),
            // 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
            hex!["7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
        ),
        (
            // 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
            hex!["547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"].into(),
            // 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
            hex!["9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"].into(),
            // 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
            hex!["5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
        ),
        (
            // 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
            hex!["f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"].into(),
            // 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
            hex!["66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"].into(),
            // 5DMa31Hd5u1dwoRKgC4uvqyrdK45RHv3CpwvpUC1EzuwDit4
            hex!["3919132b851ef0fd2dae42a7e734fe547af5a6b809006100f48944d7fae8e8ef"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
        ),
    ];

    // generated with secret: subkey inspect "$secret"/fir
    let root_key: AccountId = hex![
        // 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
        "9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
    ]
    .into();

    let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

    testnet_genesis(initial_authorities, root_key, Some(endowed_accounts), false)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::from_genesis(
        "Staging Testnet",
        "staging_testnet",
        ChainType::Live,
        staging_testnet_config_genesis,
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        None,
        None,
        Default::default(),
    )
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    enable_println: bool,
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });
    let num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 10_000 * DOLLARS;
    const STASH: Balance = 100 * DOLLARS;

    GenesisConfig {
        frame_system: Some(SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        }),
        pallet_collective_Instance1: Some(CouncilConfig::default()),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println, // this should only be enabled on development chains
                ..Default::default()
            },
        }),
        pallet_sudo: Some(SudoConfig {
            key: root_key.clone(),
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_membership_Instance1: Some(Default::default()),
        pallet_treasury: Some(Default::default()),
        pallet_society: Some(SocietyConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            pot: 0,
            max_members: 999,
        }),
        pallet_vesting: Some(Default::default()),
        pallet_assets: Some(AssetsConfig {
            account_assets: vec![],
            next_asset_id: 7u32, // start from 7, [0..6] has been reserved
            token_details: vec![],
            prices: vec![],
        }),
        pallet_convert: Some(ConvertConfig {
            convert_price: vec![
                (TokenSymbol::DOT, DOLLARS / 100),
                (TokenSymbol::KSM, DOLLARS / 100),
                (TokenSymbol::EOS, DOLLARS / 100),
            ], // initialize convert price as token = 100 * vtoken
        }),
        pallet_bridge_eos: Some(BridgeEosConfig {
            bridge_contract_account: (b"keysiancross".to_vec(), 2),
            notary_keys: initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
            // alice and bob have the privilege to sign cross transaction
            cross_chain_privilege: [(root_key.clone(), true)]
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            all_crosschain_privilege: Vec::new(),
        }),
        pallet_voucher: {
            if let Some(vouchers) = initialize_all_vouchers() {
                Some(VoucherConfig { voucher: vouchers })
            } else {
                None
            }
        },
        pallet_swap: initialize_swap_module(root_key),
    }
}

fn initialize_swap_module(sudo: AccountId) -> Option<SwapConfig> {
    /*
    This list is each token for aUSD.
    Accroding to the weight to calculate how many token will be added to the pool.
    For example, if aUSD has 10000 in the pool, DOT has to be added 10000 / (300 * dot_amount) = 15 / 15 =>
    so dot_amount = 10000 / 300 = 33.3333
    aUSD 10000
    DOT 300 aUSD
    vDOT 3 aUSD
    KSM 8.6 aUSD
    vKSM 0.086 aUSD
    EOS 2.62 aUSD
    vEOS 0.0262 aUSD
    */
    let all_pool_token = 1000 * DOLLARS;
    let count_of_supported_tokens = 7u8;
    let global_pool = {
        let pool = vec![
            (TokenSymbol::aUSD, 10000 * DOLLARS, 15),
            (
                TokenSymbol::DOT,
                (33.333_333_333_333f64 * DOLLARS as f64) as Balance,
                15,
            ), // 33.333_333_333_333
            (
                TokenSymbol::vDOT,
                (2222.222222222222f64 * DOLLARS as f64) as Balance,
                10,
            ), // 2222.222222222222
            (
                TokenSymbol::KSM,
                (1550.3875968992247f64 * DOLLARS as f64) as Balance,
                20,
            ), // 1550.3875968992247
            (
                TokenSymbol::vKSM,
                (155038.75968992253f64 * DOLLARS as f64) as Balance,
                20,
            ), // 155038.7596899225
            (
                TokenSymbol::EOS,
                (2544.529262086514f64 * DOLLARS as f64) as Balance,
                10,
            ), // 2544.529262086514
            (
                TokenSymbol::vEOS,
                (254452.9262086514f64 * DOLLARS as f64) as Balance,
                10,
            ), // 254452.9262086514
        ];
        (pool, 0)
    };
    let user_pool = {
        let pool = vec![
            (TokenSymbol::aUSD, 10000 * DOLLARS),
            (
                TokenSymbol::DOT,
                (33.333_333_333_333f64 * DOLLARS as f64) as Balance,
            ),
            (
                TokenSymbol::vDOT,
                (2222.222222222222f64 * DOLLARS as f64) as Balance,
            ),
            (
                TokenSymbol::KSM,
                (1550.3875968992247f64 * DOLLARS as f64) as Balance,
            ),
            (
                TokenSymbol::vKSM,
                (155038.75968992253f64 * DOLLARS as f64) as Balance,
            ),
            (
                TokenSymbol::EOS,
                (2544.529262086514f64 * DOLLARS as f64) as Balance,
            ),
            (
                TokenSymbol::vEOS,
                (254452.9262086514f64 * DOLLARS as f64) as Balance,
            ),
        ];
        vec![(sudo, (pool, all_pool_token))]
    };
    let swap_fee = 150;
    let exit_fee = 0;
    let total_weight = global_pool.0.iter().map(|p| p.2).collect();

    Some(SwapConfig {
        all_pool_token,
        count_of_supported_tokens,
        global_pool,
        user_pool,
        swap_fee,
        exit_fee,
        total_weight,
    })
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        true,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    let properties = {
        let mut props = serde_json::Map::new();

        props.insert(
            "ss58Format".to_owned(),
            serde_json::value::to_value(6u8)
                .expect("The ss58Format cannot be convert to json value."),
        );
        props.insert(
            "tokenDecimals".to_owned(),
            serde_json::value::to_value(12u8)
                .expect("The tokenDecimals cannot be convert to json value."),
        );
        props.insert(
            "tokenSymbol".to_owned(),
            serde_json::value::to_value("ASG".to_owned())
                .expect("The tokenSymbol cannot be convert to json value."),
        );
        Some(props)
    };
    let protocol_id = Some("keysians-test");

    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        protocol_id,
        properties,
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        false,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        local_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

/// Helper function to create GenesisConfig for keysians
pub fn keysians_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
    let num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 10_000 * DOLLARS;
    const STASH: Balance = 10_000 * DOLLARS;

    GenesisConfig {
        frame_system: Some(SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: initial_authorities
                .iter()
                .map(|k| (k.0.clone(), ENDOWMENT))
                .chain(endowed_accounts.iter().cloned().map(|x| (x, STASH / 100)))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig {
            indices: initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .chain(endowed_accounts.iter().cloned())
                .enumerate()
                .map(|accts| (accts.0 as u32, accts.1))
                .collect::<Vec<_>>(),
        }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 10,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .chain(endowed_accounts.iter().cloned())
                .collect::<Vec<_>>(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_elections_phragmen: Some(ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH / 100))
                .collect(),
        }),
        pallet_collective_Instance1: Some(CouncilConfig::default()),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            phantom: Default::default(),
        }),
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                ..Default::default()
            },
        }),
        pallet_sudo: Some(SudoConfig {
            key: root_key.clone(),
        }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_membership_Instance1: Some(Default::default()),
        pallet_treasury: Some(Default::default()),
        pallet_society: Some(SocietyConfig {
            members: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            pot: 0,
            max_members: 999,
        }),
        pallet_vesting: Some(Default::default()),
        pallet_assets: Some(AssetsConfig {
            account_assets: initialize_assets(),
            next_asset_id: 7u32, // start from 7, [0..6] has been reserved
            token_details: vec![],
            prices: vec![],
        }),
        pallet_convert: Some(ConvertConfig {
            convert_price: vec![
                (TokenSymbol::DOT, DOLLARS / 100),
                (TokenSymbol::KSM, DOLLARS / 100),
                (TokenSymbol::EOS, DOLLARS / 100),
            ], // initialize convert price as token = 100 * vtoken
        }),
        pallet_bridge_eos: Some(BridgeEosConfig {
            bridge_contract_account: (b"keysiancross".to_vec(), 3), // this eos account needs 3 signer to sign a trade
            notary_keys: initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
            // root_key has the privilege to sign cross transaction
            cross_chain_privilege: [(root_key.clone(), true)]
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            all_crosschain_privilege: Vec::new(),
        }),
        pallet_voucher: {
            if let Some(vouchers) = initialize_all_vouchers() {
                Some(VoucherConfig { voucher: vouchers })
            } else {
                None
            }
        },
        pallet_swap: initialize_swap_module(root_key),
    }
}

fn initialize_all_vouchers() -> Option<Vec<(node_primitives::AccountId, node_primitives::Balance)>>
{
    use std::collections::HashSet;

    let path = std::path::Path::join(&std::env::current_dir().ok()?, "bnc_vouchers.json");

    if !path.exists() {
        return None;
    }
    let file = std::fs::File::open(path).ok()?;
    let reader = std::io::BufReader::new(file);

    let vouchers_str: Vec<(String, String)> = serde_json::from_reader(reader).ok()?;
    let vouchers: Vec<(node_primitives::AccountId, node_primitives::Balance)> = vouchers_str
        .iter()
        .map(|v| {
            (
                parse_address(&v.0),
                v.1.parse().expect("Balance is invalid."),
            )
        })
        .collect();

    let set = vouchers.iter().map(|v| v.0.clone()).collect::<HashSet<_>>();
    let mut final_vouchers = Vec::new();
    for i in set.iter() {
        let mut sum = 0;
        for j in vouchers.iter() {
            if *i == j.0 {
                sum += j.1;
            }
        }
        final_vouchers.push((i.clone(), sum));
    }

    Some(final_vouchers)
}

fn parse_address(address: impl AsRef<str>) -> AccountId {
    let decoded_ss58 = bs58::decode(address.as_ref())
        .into_vec()
        .expect("decode account id failure");
    let mut data = [0u8; 32];
    data.copy_from_slice(&decoded_ss58[1..33]);

    node_primitives::AccountId::from(data)
}

/// initialize assets for specific keysians accounts
fn initialize_assets() -> Vec<(
    (TokenSymbol, AccountId),
    AccountAsset<Balance, Cost, Income>,
)> {
    let assets = vec![
        (
            (
                TokenSymbol::DOT,
                parse_address("5CDWwkPsyc37XdB9N5QpZosgrcqcKA48Lpb81KjDZ89W9GPm"),
            ),
            AccountAsset {
                balance: 5_000_000 * DOLLARS,
                ..Default::default()
            },
        ),
        (
            (
                TokenSymbol::KSM,
                parse_address("5DAQaLpQjAZKuX4F77Lb69e5qb3GtaKVLF1mdiYt5SAhXeLC"),
            ),
            AccountAsset {
                balance: 5_000_000 * DOLLARS,
                ..Default::default()
            },
        ),
    ];
    assets
}

/// Configure genesis for keysians test
fn keysians_config_genesis() -> GenesisConfig {
    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            // 5EHjoBVGQm4CNbdoeaozpAKGp9TTrfRuADvQ65kyiABZhYPk
            hex!["62692ff7965729b21a10862da513beb42b7da54f321e35e2147983e1e53dc935"].into(),
            // 5DSShm3qptXjE5aK7aUoVCQ7ScgCwt8wbH7MzgNwtRg4FPJZ
            hex!["3cd09eecf6faa579ff49a5bb8175c02244da1151cfa75b8b3fc9dcb15b4b281d"].into(),
            // 5G8WWBAq4gpFLEnQjsxPkfYRdXNs3UkaeR8QG39HVjub8agw
            hex!["b3d7b25de30f345bfb40e0fb78f86acc36a7edc045615d5dee2cb9539faa8219"]
                .unchecked_into(),
            // 5Dfbu9jKMdiDdMMYYnwqKLK48z3UjxwgX6JisFBmuFZ1GwBm
            hex!["46d9c4d62bf39dd4fc1e1931dd75156647729877438482a1d8910d73de116016"]
                .unchecked_into(),
            // 5Dfbu9jKMdiDdMMYYnwqKLK48z3UjxwgX6JisFBmuFZ1GwBm
            hex!["46d9c4d62bf39dd4fc1e1931dd75156647729877438482a1d8910d73de116016"]
                .unchecked_into(),
            // 5Dfbu9jKMdiDdMMYYnwqKLK48z3UjxwgX6JisFBmuFZ1GwBm
            hex!["46d9c4d62bf39dd4fc1e1931dd75156647729877438482a1d8910d73de116016"]
                .unchecked_into(),
        ),
        (
            // 5FsJoADMCmUQqmprSSqnyLPj7KAz7Kym6AzWbXMyEza7A5XH
            hex!["a83f9b156daa23ac07dd3361514d1b9f1674904d35ce8ab422bc8e1e12dac70b"].into(),
            // 5GE6M2FBBChfGfatFvRmWSgJrvSuxVYB2HNA13Fb5EFMpjst
            hex!["b819d8c01cbc46e23d9b79f7654f704a828fa1946bc8a97f56889daade1ced4e"].into(),
            // 5EaM6zor3sPnqfYLM1CRu1RFPsyMkNa1B1r3J4itBSLs8mx8
            hex!["6f13f7e727ef6b4094b346e351e66242b51fbbb6a2eac532b55389f1314d2d11"]
                .unchecked_into(),
            // 5DLXcPqxE7CDstqW9pWkKZdFWpiohiJLAmHa5qFog9qUFhgZ
            hex!["384dab202d36b0b64ee6df0a685b6eb5c0010d689447dd3d7a634fbd89adc273"]
                .unchecked_into(),
            // 5DLXcPqxE7CDstqW9pWkKZdFWpiohiJLAmHa5qFog9qUFhgZ
            hex!["384dab202d36b0b64ee6df0a685b6eb5c0010d689447dd3d7a634fbd89adc273"]
                .unchecked_into(),
            // 5DLXcPqxE7CDstqW9pWkKZdFWpiohiJLAmHa5qFog9qUFhgZ
            hex!["384dab202d36b0b64ee6df0a685b6eb5c0010d689447dd3d7a634fbd89adc273"]
                .unchecked_into(),
        ),
    ];

    // generated with secret: subkey inspect "$secret"/key
    let root_key: AccountId = hex![
        // 5CcGp66CTxesmkWKSGyWjGTxz53ebsRgAPQK8tjAGufJTsCF
        "18138bb1ff07235d2f4de5adda22c919f03fc2580e563aec374556473ec28a3a"
    ]
    .into();

    let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

    keysians_genesis(initial_authorities, root_key, endowed_accounts)
}

/// Keysians testnet config.
pub fn keysians_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::from_genesis(
        "Keysians Testnet",
        "keysians_testnet",
        ChainType::Live,
        keysians_config_genesis,
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        None,
        None,
        Default::default(),
    )
}
