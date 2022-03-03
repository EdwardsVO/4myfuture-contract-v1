use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::serde::Deserialize;
use near_sdk::collections::UnorderedMap;
use near_sdk::{json_types::U128, env, near_bindgen, AccountId, Balance, Promise};
//use std::collections::HashMap;

near_sdk::setup_alloc!();


//--------------------------------- APP OBJECTS --------------------------//
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Contribution {
    contribution_id: i128,
    proposal_id: i128,
    proposal_pic: String,
    amount: u128,
    user_funded: String,
    user_pic: String,
    date: String,
    comments: String 
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Payment {
    to: String,
    by: String,
    amount: u128,
    date: String,
    pay_type: String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    id: String,
    contributions: Vec<Contribution>,
    with_active_proposal: bool,
    rank: i128,
    picture: String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    user: String,
    amount_needed: u128,
    funds: u128,
    title: String,
    description: String,
    goal: String,
    link_institution: String,
    link_pensum: String,
    init_date: String,
    finish_date: String,
    photos: Vec<String>,
    status: i128,
    index: i128
}
//--------------------------------- APP OBJECTS --------------------------//

//--------------------------------- CONTRACT STORAGE --------------------------//
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ForMyFuture {
    //Users registered
    pub users: UnorderedMap<AccountId, User>,

    //Proposals made it
    pub proposals: UnorderedMap<i128, Proposal>,

    //Contributions made it
    pub contributions: UnorderedMap<i128, Contribution>,

    //Payments within the contract
    pub payments: UnorderedMap<i128, Payment>
}
//--------------------------------- CONTRACT STORAGE --------------------------//


impl Default for ForMyFuture {
    fn default() -> Self {
        Self {
            users: UnorderedMap::new(b"a"),
            proposals: UnorderedMap::new(b"b"),
            contributions: UnorderedMap::new(b"c"),
            payments: UnorderedMap::new(b"p")            
        }
    }
}


//--------------------------------- CONTRACT MAIN --------------------------//
#[near_bindgen]
impl ForMyFuture {


    //Function to log an user into the app, if she/he don't exist will be created
    pub fn login(&mut self) -> User {
        let signer  = env::signer_account_id().to_string();
        if self.users.get(&signer).is_none() {
            let user = User {
                id: env::signer_account_id().to_string(),
                contributions: Vec::new(),
                with_active_proposal: false,
                rank: 0,
                picture: String::from("")
            };
            self.users.insert(&signer, &user);
        }
        let user_r = self.users.get(&signer);
        user_r.unwrap()
    }


}