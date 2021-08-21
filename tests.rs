// This file is part of Substrate-node-template.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// Test for create-claim working.
#[test]
fn create_claim_works() {
	new_test_ext().execute_with( || {
		let claim:Vec<u8> = vec![0,1];

		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(1,frame_system::Pallet::<Test>::block_number())
		);
	})
}

// Test for create-claim-error: ClaimAlreadyClaimed.
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with( || {
		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::ClaimAlreadyClaimed
		);
	})
}

// Test for create-claim-error: OverFlow.
#[test]
fn create_claim_failed_when_length_overflow() {
	new_test_ext().execute_with( || {
		let claim:Vec<u8> = vec![100];
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::OverFlow
		);
	})
}

// Test for revoke-claim working.
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with( || {
		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),Proofs::<Test>::get(&claim));
	})
}

// Test for revoke-claim-error: NoSuchClaim.
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with( || {
		let claim:Vec<u8> = vec![0,1];

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
			Error::<Test>::NoSuchClaim
		);
	})
}

// Test for revoke-claim-error: NotClaimOwner.
#[test]
fn revoke_claim_failed_when_sender_is_not_owner() {
	new_test_ext().execute_with( || {
		let claim:Vec<u8> = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
	
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2),claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

// Test for transfer-claim working.
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with( || {
		let claim:Vec<u8> = vec![0,1];
		let new_owner:u64 = 2;
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(),new_owner.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(2,frame_system::Pallet::<Test>::block_number())
		);
	})
}

// Test for transfer-claim-error: NotClaimOwner.
#[test]
fn transfer_claim_failed_when_sender_not_owner() {
	new_test_ext().execute_with( || {
		let claim:Vec<u8> = vec![0,1];
		let new_owner:u64 = 2;
		let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2),claim.clone(),new_owner.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}