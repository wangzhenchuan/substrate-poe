use crate::{mock::*, Error};
use super::*;
use frame_support::{assert_ok, assert_noop};

const KITTY_NAME: [u8; 8] = *b"test1111";
const ACCOUNT_BALANCE: u128 = 1000000000;

#[test]
fn it_works_for_create(){
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        Balances::force_set_balance(RuntimeOrigin::root(), account_id, ACCOUNT_BALANCE);

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id );
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), KITTY_NAME));

		let k = KittiesModule::kitties(kitty_id).unwrap();
        System::assert_has_event(Event::KittyCreated { who: account_id, kitty_id: kitty_id, kitty: k }.into());

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1 );
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id), KITTY_NAME),
            Error::<Test>::InvalidKittyId
        );
    });
}

#[test]
fn it_works_for_breed(){
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        Balances::force_set_balance(RuntimeOrigin::root(), account_id, ACCOUNT_BALANCE);

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, KITTY_NAME),
            Error::<Test>::SameKittyId
        );

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1, KITTY_NAME),
            Error::<Test>::InvalidKittyId
        );

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), KITTY_NAME));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), KITTY_NAME));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        assert_ok!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), 
            kitty_id, 
            kitty_id + 1,
            KITTY_NAME
        ));

        let breed_kitty_id = 2;
        let k = KittiesModule::kitties(breed_kitty_id).unwrap();

        System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyBred { who: account_id, kitty_id: breed_kitty_id, kitty: k } ));

        assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
        assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
        assert_eq!(
            KittiesModule::kitty_parents(breed_kitty_id),
            Some((kitty_id, kitty_id + 1))
        );
    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let recepient = 2;

        Balances::force_set_balance(RuntimeOrigin::root(), account_id, ACCOUNT_BALANCE);

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), KITTY_NAME));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        assert_noop!(
            KittiesModule::transfer(RuntimeOrigin::signed(recepient), recepient, kitty_id),
            Error::<Test>::NotOwner
        );

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recepient, kitty_id));

        System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyTransferred { who: account_id, to: recepient, kitty_id: kitty_id } ));

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recepient));

        assert_ok!(
            KittiesModule::transfer(RuntimeOrigin::signed(recepient), account_id, kitty_id)
        );

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
    });
}

#[test]
fn it_works_for_sale() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        Balances::force_set_balance(RuntimeOrigin::root(), account_id, ACCOUNT_BALANCE);

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id );
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), KITTY_NAME));

        // sale ok
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id, 500));

        // not owner
        assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(account_id + 1), kitty_id, 500), Error::<Test>::NotOwner);

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), KITTY_NAME));

        // sale ok
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), (kitty_id + 1), 500));
    });
}

#[test]
fn it_works_for_buy() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let account_id2 = 2;
        let recepient = 2;

        // add balance
        assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, ACCOUNT_BALANCE));
        assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id2, ACCOUNT_BALANCE));

        // 
        assert_noop!(
            KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id), Error::<Test>::InvalidKittyId
        );

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), KITTY_NAME));
        assert_eq!(Balances::free_balance(account_id), ACCOUNT_BALANCE - EXISTENTIAL_DEPOSIT * 10);

        // already owned
        assert_noop!(
            KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id),
            Error::<Test>::AlreadyOwned
        );

        // not on sale
        assert_noop!(
            KittiesModule::buy(RuntimeOrigin::signed(account_id2), kitty_id),
            Error::<Test>::NotOnSale
        );

        // 
        assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id, 500));
        assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(account_id2), kitty_id));
        System::assert_last_event(Event::KittyBought { who: account_id2, kitty_id: 0 }.into());
    });
}
