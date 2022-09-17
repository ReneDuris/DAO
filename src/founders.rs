elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait Founders: crate::storage::Storage
{
    //CHANGE TOKEN FOR PAYMENTS
    #[only_owner]
    #[endpoint(changeToken)]
    fn change_token(&self,new_token_id : EgldOrEsdtTokenIdentifier){
        self.token_id().set(new_token_id);
    }
//CHANGE COST OF VOTING
    #[only_owner]
    #[endpoint(changeCostVote)]
    fn change_cost_vote(&self,new_cost_vote : BigUint){
        require!(new_cost_vote > BigUint::zero(), "Cost is too low.");
        self.cost_vote().set(new_cost_vote);
    }
//CHANGE COST OF ACTIVATING PROPOSALS
    #[only_owner]
    #[endpoint(changeCostProposals)]
    fn change_cost_proposals(&self,new_cost_vote : BigUint){
        require!(new_cost_vote > BigUint::zero(), "Cost is too low.");
        self.cost_proposals().set(new_cost_vote);
    }

//SET EXPIRATION TIME FOR PROPOSALS
    #[only_owner]
    #[endpoint(setExpirationTime)]
    fn set_expiration_time(&self,seconds:u64){
        self.expiration_time().set(seconds);
    }

     //APPROVE PROPOSAL TO BEGIN VOTING, OTHERWISE PROPOSAL IS NOT ACTIVE. FALSE AS DEFAULT.
     #[only_owner]
     #[endpoint(proposalPush)]
     fn proposal_push(&self,proposal_id:ManagedBuffer){
         let current_timestamp = self.blockchain().get_block_timestamp();
         let expiration_time = self.expiration_time().get();
         let future_expiration = current_timestamp + expiration_time;
         
         require!(self.proposal_struct(&proposal_id).is_empty() == false, "Proposal not found.");
         require!(self.proposal_start(&proposal_id).is_empty() == true, "Already pushed.");
         require!(self.proposal_activated(&proposal_id).get() == true, "Proposal wasnt activated yet.");
 
         self.proposal_pushed(&proposal_id).set(true);
         self.proposal_start(&proposal_id).set(future_expiration);
     }
}