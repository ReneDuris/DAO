#![no_std]
elrond_wasm::imports!();
const APPROVE : &[u8] = "approve".as_bytes();
const REJECT : &[u8] = "reject".as_bytes();
pub mod storage;
pub mod founders;
#[elrond_wasm::contract]
pub trait Dao: 
          storage::Storage
         +founders::Founders {
    #[init]
    fn init(&self,seconds: u64,token_id : EgldOrEsdtTokenIdentifier, cost_vote : BigUint,cost_proposals: BigUint) {
        self.expiration_time().set(seconds);
        self.token_id().set(token_id);
        self.cost_vote().set(cost_vote);
        self.cost_proposals().set(cost_proposals);
    }

     //CREATING PROPOSAL
     #[endpoint(createProposal)]
     fn create_proposal(&self,proposal_content:ManagedBuffer,opt: MultiValueEncoded<ManagedBuffer>)-> SCResult<ManagedBuffer>{
         let proposal_creator = self.blockchain().get_caller();
         let creation_timestamp = self.blockchain().get_block_timestamp();
         let owner = self.blockchain().get_owner_address();
 
         let proposal_id = self.create_hash();
         use crate::storage::Proposal;
         let proposal_struct = Proposal{
             proposal_id : proposal_id.clone(),
             proposal_creator : proposal_creator.clone(),
             proposal_content,
             creation_timestamp
         };
         self.proposal_struct_user(&proposal_id, &proposal_creator).set(&proposal_struct);
         self.proposal_struct(&proposal_id).set(proposal_struct);
 
 if proposal_creator == owner {
     self.proposal_activated(&proposal_id).set(true);
     self.proposal_push(proposal_id.clone());
 }
 else{
         self.proposal_pushed(&proposal_id).set(false);
         self.proposal_activated(&proposal_id).set(false);
     }
 
             for buffer in opt {
                 self.created_id(&proposal_id).update(|id| *id += 1usize);
                 let option_id = self.created_id(&proposal_id).get();
                 self.storage_options_id(&proposal_id).insert(option_id);
                 self.proposal_options(&proposal_id,option_id).set(buffer);
             }
             if self.storage_options_id(&proposal_id).is_empty() == false {
             require!(self.storage_options_id(&proposal_id).len() >= 2usize && self.storage_options_id(&proposal_id).len() <= 5usize, "You have to set atleast 2 options.");
             }
         Ok(proposal_id)
     
     }

//ACTIVATE PROPOSAL TO BE REVIEWED BY TEAM (NOT READY FOR VOTE,FIRST IT NEEDS TO BE ACTIVATED,THEN PUSHED)
    #[payable("*")]
    #[endpoint(proposalActivate)]
    fn proposal_activate(&self,proposal_id:ManagedBuffer){
        let caller = self.blockchain().get_caller();
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();

        require!(payment_token == self.token_id().get(), "Wrong token sent.");
        require!(payment_amount == self.cost_proposals().get(), "Sent amount is different from what actual cost is.");
        require!(self.proposal_struct(&proposal_id).is_empty() == false, "Proposal not found.");
        require!(self.proposal_activated(&proposal_id).get() == false, "Proposal already activated.");
        
        let proposal =self.proposal_struct(&proposal_id).get();
        let proposal_creator = proposal.proposal_creator;

        require!(caller == proposal_creator, "You didnt created this proposal.");

        if self.created_id(&proposal_id).is_empty() == false {
        }
        self.proposal_activated(&proposal_id).set(true);
    }
//DELETE PROPOSAL
    #[endpoint(proposalDelete)]
    fn proposal_delete(&self,proposal_id:ManagedBuffer){
        let caller = self.blockchain().get_caller();
        let proposal =self.proposal_struct(&proposal_id).get();
        let proposal_creator = proposal.proposal_creator;

        require!(caller == proposal_creator, "You didnt created this proposal.");
        require!(self.proposal_pushed(&proposal_id).get() == false, "Cannot deleted pushed proposal.");

        self.proposal_struct_user(&proposal_id, &proposal_creator).clear();
        self.proposal_struct(&proposal_id).clear();
        self.proposal_activated(&proposal_id).clear();
        self.storage_options_id(&proposal_id).clear();


    }

    #[payable("*")]
    #[endpoint(voteProposal)]
    fn vote_on_proposal(&self,proposal_id:ManagedBuffer, vote: ManagedBuffer){
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let (payment_token, payment_amount) = self.call_value().egld_or_single_fungible_esdt();

        require!(payment_token == self.token_id().get(), "Wrong token sent.");
        require!(payment_amount == self.cost_vote().get(), "Sent amount is different from what actual cost is.");
        require!(self.proposal_struct(&proposal_id).is_empty() == false, "Proposal not found.");
        require!(self.proposal_pushed(&proposal_id).get() == true, "Not pushed to voting.");
        require!(self.already_voted(&proposal_id).contains(&caller) == false, "Already voted");

        let future_expiration = self.proposal_start(&proposal_id).get();

        require!(current_timestamp < future_expiration, "Proposal ended");

        if self.created_id(&proposal_id).is_empty() == true {
          let approve = ManagedBuffer::new_from_bytes(APPROVE);
          let reject = ManagedBuffer::new_from_bytes(REJECT);

            require!(vote== approve || vote == reject, "Wrong vote format.");

        }
        else{
           let mut data_buffer = [0u8; 10usize];
           let bytes_array = vote.load_to_byte_array(&mut data_buffer);
           let vote_num:u32 = (bytes_array[0] as char).to_digit(10).unwrap_or(0u32);
           let vote_num_usize = vote_num as usize;
        
            require!(self.storage_options_id(&proposal_id).contains(&vote_num_usize) == true, "This option does not exist.");
        }
        self.already_voted(&proposal_id.clone()).insert(caller.clone());
        self.vote_count(proposal_id.clone(),vote.clone()).update(|vote| *vote += 1usize);

        use crate::storage::ProposalVote;
        let proposal_vote = ProposalVote{
            user: caller,
            vote,
            vote_time: current_timestamp
        };
        self.vote_address(proposal_id).insert(proposal_vote);
    }
    
}
