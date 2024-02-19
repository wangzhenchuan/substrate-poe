use crate::{mock::*, Error};
use super::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_create(){
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id );
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		let k = KittiesModule::kitties(kitty_id).unwrap();
        System::assert_has_event(Event::KittyCreated { who: account_id, kitty_id: kitty_id, kitty: k }.into());

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1 );
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id)),
            Error::<Test>::InvalidKittyId
        );
    });
}

#[test]
fn it_works_for_breed(){
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
            Error::<Test>::SameKittyId
        );

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
            Error::<Test>::InvalidKittyId
        );

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        assert_ok!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), 
            kitty_id, 
            kitty_id + 1
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

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
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

// // test for event value
// #[test]
// fn it_works_for_event() {
//     new_test_ext().execute_with(|| {
//         let kitty_id = 0;
//         let account_id = 1;
//         let recepient = 2;

// 		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

// 		let breed_kitty_id = 0;
// 		let k = KittiesModule::kitties(breed_kitty_id).unwrap();
// 		System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyCreated { who: account_id, kitty_id: breed_kitty_id, kitty: k }));

// 		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recepient, kitty_id));

// 		System::assert_has_event(RuntimeEvent::KittiesModule(Event::KittyTransferred { who:account_id, to: recepient, kitty_id }));
//     });
// }