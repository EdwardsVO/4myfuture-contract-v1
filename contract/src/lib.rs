use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::serde::Deserialize;
use near_sdk::collections::UnorderedMap;
use near_sdk::{json_types::U128, env, near_bindgen, AccountId, Balance, Promise};
use std::time::{Duration, Instant};
//use std::collections::HashMap;

near_sdk::setup_alloc!();


//--------------------------------- CONSTANTS --------------------------//
const ONE_NEAR: Balance = 1000000000000000000000000;
const BASE_TO_CONVERT: u64 = 1000000;
const NANOSEC_SEC: u64 = 1000000000;
const NANOSEC_MIN: u64 = 60000000000;
const NANOSEC_HOR: u64 = 3600000000000;
const NANOSEC_DIA: u64 = 86400000000000;


//--------------------------------- APP OBJECTS --------------------------//
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Contribution {
    contribution_id: i128,
    proposal_id: i128,
    amount: u128,
    to: String,
    by: String,
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

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
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
    init_date: u64,
    finish_date: u64,
    pics: Vec<String>,
    status: i128,
    index: i128,
    is_reclaimable: bool
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


    /*******************************/
    /******* USER FUNCTIONS  ********/
    /*******************************/

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

    //Function to return all users registered in the contract
    pub fn get_users(self) -> Vec<User> {
        let user_list = self.users.values_as_vector().to_vec();
        user_list
    }

    //Function to get one user registered
    pub fn get_user(self, user_id: AccountId) -> User {
        assert!(self.users.get(&user_id).is_some(), "User not registered");
        let user = self.users.get(&user_id);
        user.unwrap()
    }



    /*******************************/
    /******* PROPOSAL FUNCTIONS  ********/
    /*******************************/    

    //Function to create one proposal
    pub fn create_proposal(&mut self, 
        title: String, 
        goal: String,
        link_institution: String,
        link_pensum: String,
        pics: Vec<String>,
        amount_needed: u128,
        description: String, 
        finish_date: u64) -> Proposal {
            let user_requesting = env::signer_account_id().to_string();
            let user = self.users.get(&user_requesting);
            let mut user_update = user.unwrap();
            assert!(amount_needed > (0), "Invalid amount needed");
            assert!(self.users.get(&user_requesting).is_some(), "User not loged");
            assert!(user_update.with_active_proposal == false, "User already have one proposal");
            let amount_yocto = amount_needed * ONE_NEAR;
            let index = i128::from(self.proposals.len() + 1);
            let proposal = Proposal {
                title: title.to_string(),
                user: user_requesting,
                status: i128::from(0),
                goal: goal,
                link_institution: link_institution.to_string(),
                link_pensum: link_pensum.to_string(),
                pics: pics,
                amount_needed: amount_yocto,
                description: description,
                init_date: env::block_timestamp(),
                finish_date: finish_date,
                funds: 0,
                index: index, 
                is_reclaimable: false
            };
            user_update.with_active_proposal = true;
            self.users.insert(&user_update.id, &user_update);
            self.proposals.insert(&proposal.index, &proposal);
            proposal
    }

    //Get one proposal
    pub fn get_proposal(self, proposal_id: i128) -> Proposal {
        assert!(proposal_id <= i128::from(self.proposals.len() + 1), "Invalid proposal id");
        let proposal = self.proposals.get(&proposal_id);
        proposal.unwrap()
    }

    //Inactive Proposal 
    pub fn pause_proposal(&mut self, proposal_id: i128) -> Proposal {
        assert!(proposal_id <= i128::from(self.proposals.len() + 1), "Invalid proposal id");
        assert!(env::signer_account_id() == "4myfuture.sputnikv2.testnet".to_string(), "Signer unauthorized to call this function");
        let mut proposal = self.proposals.get(&proposal_id).unwrap();
        proposal.status = 2;
        self.proposals.insert(&proposal_id, &proposal);
        proposal
    }

    //Get all proposals
    pub fn get_proposals(self) -> Vec<Proposal> {
        let proposal_list = self.proposals.values_as_vector().to_vec();
        proposal_list
    }

    //Get the current funds in realtion to the funds needed in percentage 
    pub fn get_proposal_funds_percentage(self, proposal_id: i128) -> u128 {
        assert!(proposal_id <= i128::from(self.proposals.len() + 1), "Invalid proposal id");
        let proposal = self.proposals.get(&proposal_id).unwrap();
        let percentage =  (proposal.funds*100)/proposal.amount_needed;
        percentage
    }

    /*******************************/
    /******* CONTRIBUTION FUNCTIONS  ********/
    /*******************************/    
    

    #[payable]
    pub fn contribute(&mut self, proposal_id: i128, comments: String) -> Contribution{
        assert!(proposal_id <= i128::from(self.proposals.len() + 1), "Invalid proposal id");
        assert!(env::attached_deposit() > 0, "Invalid contribution amount");
        let mut proposal = self.proposals.get(&proposal_id).unwrap();
        assert!(env::attached_deposit() <= proposal.amount_needed, "Contribution higher than required");
        assert!(proposal.status == 0, "Can't contribute to this proposal");
        let percentage =  (proposal.funds*100)/proposal.amount_needed;
        assert!(percentage <= 100, "The contribution exceeded the proposal goal");
        assert!(env::block_timestamp() < u64::from(proposal.finish_date), "Proposal already ended");
        if self.users.get(&env::signer_account_id().to_string()).is_none() { //FIXME
            env::log(b"USUARIO NO REGISTRADO");
            let _user_just_registered = self.login();
        }
        let mut user = self.users.get(&env::signer_account_id()).unwrap();
        let index = i128::from(self.contributions.len() + 1);
        let contribution = Contribution {
            contribution_id: index,
            proposal_id: proposal.index,
            amount: env::attached_deposit(),
            by: env::signer_account_id(),
            to: proposal.clone().user,
            date: env::block_timestamp().to_string(),
            comments: comments 
        };
        self.contributions.insert(&i128::from(self.contributions.len()+1), &contribution);
        proposal.funds += env::attached_deposit();
        let new_percentage = (proposal.funds*100)/proposal.amount_needed;
        if new_percentage >= 75 {
            proposal.is_reclaimable = true;
        }
        user.contributions.push(contribution.clone());
        self.proposals.insert(&proposal.index, &proposal);
        self.users.insert(&user.id, &user);
        return contribution;
    }

    pub fn reclaim_funds(&mut self, proposal_id: i128) -> Proposal {
        assert!(proposal_id <= i128::from(self.proposals.len() + 1), "Invalid proposal id");
        let mut proposal = self.proposals.get(&proposal_id).unwrap();
        assert!(env::signer_account_id() == proposal.user, "Only owner can reclaim funds");
        assert!(proposal.status == 0, "Can't reclaim funds for this proposal");    
        //let percentage = &self.get_proposal_funds_percentage(proposal_id);     
        let percentage = (proposal.funds*100)/proposal.amount_needed;
        assert!(percentage > 75, "You can't reclaim your funds yet");
        //assert in finish time
        let owner_id = &proposal.user;
        let mut owner = self.users.get(&owner_id).unwrap();
        Promise::new(owner_id.clone()).transfer(proposal.funds);
        proposal.status = 1;
        owner.with_active_proposal = false;
        self.proposals.insert(&proposal.index, &proposal);
        self.users.insert(&owner_id, &owner);
        proposal
    }

    /*******************************/
    /******* CONTRIBUTION FUNCTIONS  ********/
    /*******************************/    
    


}