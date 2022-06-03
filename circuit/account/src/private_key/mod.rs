// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

pub mod to_compute_key;

#[cfg(test)]
use snarkvm_circuit_types::environment::assert_scope;

use crate::ComputeKey;
use snarkvm_circuit_network::Aleo;
use snarkvm_circuit_types::{environment::prelude::*, Scalar};

pub struct PrivateKey<A: Aleo> {
    /// The signature secret key.
    sk_sig: Scalar<A>,
    /// The signature secret randomizer.
    r_sig: Scalar<A>,
    /// The VRF secret key.
    sk_vrf: Scalar<A>,
}

#[cfg(console)]
impl<A: Aleo> Inject for PrivateKey<A> {
    type Primitive = (A::ScalarField, A::ScalarField, A::ScalarField);

    /// Initializes an account private key from the given mode and `(sk_sig, r_sig, sk_vrf)`.
    fn new(mode: Mode, (sk_sig, r_sig, sk_vrf): Self::Primitive) -> Self {
        Self { sk_sig: Scalar::new(mode, sk_sig), r_sig: Scalar::new(mode, r_sig), sk_vrf: Scalar::new(mode, sk_vrf) }
    }
}

impl<A: Aleo> PrivateKey<A> {
    /// Returns the signature secret key.
    pub fn sk_sig(&self) -> &Scalar<A> {
        &self.sk_sig
    }

    /// Returns the signature randomizer.
    pub fn r_sig(&self) -> &Scalar<A> {
        &self.r_sig
    }

    /// Returns the VRF secret key.
    pub fn sk_vrf(&self) -> &Scalar<A> {
        &self.sk_vrf
    }
}

#[cfg(console)]
impl<A: Aleo> Eject for PrivateKey<A> {
    type Primitive = (A::ScalarField, A::ScalarField, A::ScalarField);

    ///
    /// Ejects the mode of the account private key.
    ///
    fn eject_mode(&self) -> Mode {
        (&self.sk_sig, &self.r_sig, &self.sk_vrf).eject_mode()
    }

    ///
    /// Ejects the account private key as `(sk_sig, r_sig, sk_vrf)`.
    ///
    fn eject_value(&self) -> Self::Primitive {
        (&self.sk_sig, &self.r_sig, &self.sk_vrf).eject_value()
    }
}