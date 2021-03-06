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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::collections::btree_map::BTreeMap;
use core::convert::TryInto;
use frame_support::traits::Get;
use frame_support::{
    debug, decl_error, decl_event, decl_module, decl_storage, ensure, IterableStorageMap, Parameter,
};
use frame_system::{self as system, ensure_root};
use sp_runtime::traits::{AtLeast32Bit, MaybeSerializeDeserialize, Member, StaticLookup, Zero};

pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    /// BNC Balance
    type Balance: Member
        + Parameter
        + Default
        + From<u128>
        + AtLeast32Bit
        + MaybeSerializeDeserialize
        + Copy
        + Zero
        + Into<u128>;
}

decl_event! {
    pub enum Event<T> where <T as system::Trait>::AccountId, <T as Trait>::Balance {
        /// A event indicate user receives transaction.
        IssuedVoucher(AccountId, Balance),
        DestroyedVoucher(AccountId, Balance),
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Transferring too big balance
        TransferringTooBigBalance,
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Voucher {
        /// How much voucher you have
        pub BalancesVoucher get(fn voucher) config(): map hasher(blake2_128_concat) T::AccountId => T::Balance;
        /// Total BNC in mainnet
        TotalSuppliedBNC get(fn toal_bnc): T::Balance = (80_000_000u128 * 10u128.pow(12)).try_into().map_err(|_| "failed to u128 conversion").unwrap();
        /// Current remaining BNC adds all others vouchers, equaling to TotalSuppliedBNC
        RemainingBNC get(fn remaining_bnc): T::Balance = (80_000_000u128 * 10u128.pow(12)).try_into().map_err(|_| "failed to u128 conversion").unwrap();
    }
    add_extra_genesis {
        build(|config: &GenesisConfig<T>| {
            // initialize all vouchers for each register
            let mut total = 0.into();
            for (who, balance) in &config.voucher {
                <BalancesVoucher<T>>::insert(who, balance);
                total += *balance;
            }
            let left = <TotalSuppliedBNC<T>>::get() - total;
            <RemainingBNC<T>>::put(left);
        });
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = T::DbWeight::get().reads_writes(1, 1)]
        pub fn issue_voucher(
            origin,
            dest: <T::Lookup as StaticLookup>::Source,
            #[compact]
            amount: T::Balance,
        ) {
            ensure_root(origin)?;

            let balance = <RemainingBNC<T>>::get();
            ensure!(balance >= amount, Error::<T>::TransferringTooBigBalance);

            // ensure this address added into keysians node
            let dest = T::Lookup::lookup(dest)?;

            // do not send balances for a account multiple times, just for one time
            if <BalancesVoucher<T>>::contains_key(&dest) {
                // increase balances for this account
                <BalancesVoucher<T>>::mutate(&dest, |balance| {
                    *balance += amount;
                });
            } else {
                <BalancesVoucher<T>>::insert(&dest, amount);
            }

            // reduce from total BNC
            <RemainingBNC<T>>::mutate(|balance| {
                    *balance -= amount;
            });

            Self::deposit_event(RawEvent::IssuedVoucher(dest, amount));
        }

        #[weight = T::DbWeight::get().writes(1)]
        fn intialize_all_voucher(origin) {
            ensure_root(origin)?;

            let total = TotalSuppliedBNC::<T>::get();
            <RemainingBNC<T>>::mutate(|balance| {
                *balance = total;
            });

            for (who, _) in <BalancesVoucher<T>>::iter() {
                <BalancesVoucher<T>>::mutate(&who, |balance| {
                    *balance = 0.into();
                });
            }
        }

        #[weight = T::DbWeight::get().writes(1)]
        pub fn destroy_voucher(
            origin,
            dest: <T::Lookup as StaticLookup>::Source,
            #[compact]
            amount: T::Balance,
        ) {
            ensure_root(origin)?;

            // ensure this address added into keysians node
            let dest = T::Lookup::lookup(dest)?;

            let owner_balances = <BalancesVoucher<T>>::get(&dest);

            ensure!(owner_balances >= amount, Error::<T>::TransferringTooBigBalance);

            // do not send balances for a account multiple times, just for one time
            if <BalancesVoucher<T>>::contains_key(&dest) {
                // desotry balances for this account
                if owner_balances >= amount {
                    <BalancesVoucher<T>>::mutate(&dest, |balance| {
                        *balance -= amount;
                    });
                    // add back to total BNC
                    let remaining = RemainingBNC::<T>::get();
                    if remaining + amount <= TotalSuppliedBNC::<T>::get() {
                        <RemainingBNC<T>>::mutate(|balance| {
                                *balance += amount;
                        });
                    }
                }
            } else {
                ();
            }

            Self::deposit_event(RawEvent::DestroyedVoucher(dest, amount));
        }

        #[weight = T::DbWeight::get().reads_writes(1, 1)]
        pub fn export_all_vouchers(origin) {
            ensure_root(origin)?;

            let mut vouchers = BTreeMap::new();
            for (who, balance) in <BalancesVoucher<T>>::iter() {
                vouchers.insert(who, balance);
            }
            #[cfg(feature = "std")]
            {
                use std::io::prelude::*;
                if let Ok(ref current_path) = std::env::current_dir() {
                    let vouchers_path = std::path::Path::join(current_path, "all_vouchers.json");
                    match (std::fs::File::create(vouchers_path), serde_json::to_vec(&vouchers)) {
                        (Ok(ref mut file), Ok(ref bytes)) => {
                            if !file.write_all(&bytes[..]).is_ok() {
                                debug::warn!("failed to export all vouchers");
                            }
                        }
                        _ => debug::warn!("failed to export all vouchers"),
                    }
                }
            }
        }
    }
}
