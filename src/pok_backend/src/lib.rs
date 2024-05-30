use agreement::Agreement;
use candid::Principal;
use helpers::ToUser;
use user::{Agree, CreateAgreement, User};
mod agreement;
mod helpers;
mod signature;
mod user;

impl ToUser for Principal {
    fn principal_to_user(self) -> User {
        User { identity: self }
    }
}

fn _create_new_agreement(terms: Vec<String>, with_user: Principal) -> Agreement {
    let creator = Principal::principal_to_user(ic_cdk::caller());

    let agreement = creator.clone().new_agreement(
        terms,
        String::from("new date"),
        Principal::principal_to_user(with_user),
    );
    creator.agree(agreement)
}
fn _agree_to_agreement(user: Principal, agreement: Agreement) -> Agreement {
    let agreeing_party = Principal::principal_to_user(user);
    agreeing_party.agree(agreement)
}
