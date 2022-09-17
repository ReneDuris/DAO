elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct Proposal<M: ManagedTypeApi> {
    pub proposal_id: ManagedBuffer<M>,
    pub proposal_creator: ManagedAddress<M>,
    pub proposal_content: ManagedBuffer<M>,
    pub creation_timestamp: u64
}
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode,NestedDecode)]
pub struct ProposalVote<M: ManagedTypeApi> {
    pub user: ManagedAddress<M>,
    pub vote: ManagedBuffer<M>,
    pub vote_time: u64
}

#[elrond_wasm::module]
pub trait Storage{

     //generating porposal ID
     fn create_hash(&self)-> ManagedBuffer{
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        let rand_hash = rand_source.next_bytes(32);
        return rand_hash;
    }  
// PROPOSAL STRUCT, CONTAINS ALL DATA OF SPECIFIC USER AND SPECIFIC USER
    #[view(getProposalStructUser)]
    #[storage_mapper("proposalStructUser")]
    fn proposal_struct_user(&self,proposal_id: &ManagedBuffer, user: &ManagedAddress) -> SingleValueMapper<Proposal<Self::Api>>;
//STRUCT OF PROPOSAL SAVED INTO PROPOSAL ID
    #[view(getProposalStruct)]
    #[storage_mapper("proposalStruct")]
    fn proposal_struct(&self,proposal_id: &ManagedBuffer) -> SingleValueMapper<Proposal<Self::Api>>;
//EXPIRATION TIME OF PROPOSAL AFTER PUSHED
    #[view(getProposalstart)]
    #[storage_mapper("proposalstart")]
    fn proposal_start(&self,proposal_id: &ManagedBuffer) -> SingleValueMapper<u64>;
// SECONDS OF PROPOSAL TIME.
    #[view(getexpirationDate)]
    #[storage_mapper("expirationDate")]
    fn expiration_time(&self) -> SingleValueMapper<u64>;
//PUSH PROPOSAL LIVE
    #[view(getProposalState)]
    #[storage_mapper("ProposalPushed")]
    fn proposal_pushed(&self,proposal_id: &ManagedBuffer) -> SingleValueMapper<bool>;
//PROPOSAL ACTIVATED FOR COMMUNITY(FOUNDER WILL SKIP THIS STEP.)
    #[view(getProposalActivate)]
    #[storage_mapper("ProposalActivated")]
    fn proposal_activated(&self,proposal_id: &ManagedBuffer) -> SingleValueMapper<bool>;
//COUNTING VOTE FOR EACH TYPE AND STATE
    #[view(getVoteCount)]
    #[storage_mapper("voteCount")]
    fn vote_count(&self,proposal_id: ManagedBuffer,vote:ManagedBuffer) -> SingleValueMapper<usize>;
//STORED VOTE ON SPECIFIC PROPOSAL
    #[view(getVoteAddress)]
    #[storage_mapper("voteAddress")]
    fn vote_address(&self,proposal_id: ManagedBuffer) -> UnorderedSetMapper<ProposalVote<Self::Api>>;
//STORED ALL ADDRESSES VOTED ON SPECIFIC PROPOSAL
    #[storage_mapper("alreadyVoted")]
    fn already_voted(&self,proposal_id: &ManagedBuffer) -> UnorderedSetMapper<ManagedAddress>;
//GROWING ID EACH TIME OPTIONS IS ADDED (TO DO NOT HAVE SAME ID) IN SPECIFIC PROPOSAL
    #[view(getCreatedId)]
    #[storage_mapper("createdId")]
    fn created_id(&self,proposal_id: &ManagedBuffer) -> SingleValueMapper<usize>;
//STORE OPTIONS INPUT STRING BY CREATOR OF PROPOSAL AND ID ASSIGNED
    #[view(proposalOptions)]
    #[storage_mapper("proposalOptions")]
    fn proposal_options(&self,proposal_id: &ManagedBuffer,proposal_option: usize) -> SingleValueMapper<ManagedBuffer>;
//STORING ALL ID INTO SPECIFIC PROPOSAL
    #[view(optionsID)]
    #[storage_mapper("OptionsID")]
    fn storage_options_id(&self,proposal_id: &ManagedBuffer) -> UnorderedSetMapper<usize>;
//TOKEN USED FOR PAYMENTS
    #[view(tokenID)]
    #[storage_mapper("tokenID")]
    fn token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;
//COST OF SINGLE VOTE
    #[view(costVote)]
    #[storage_mapper("costVote")]
    fn cost_vote(&self) -> SingleValueMapper<BigUint>;
//COST OF CREATING PROPOSAL
    #[view(costProposals)]
    #[storage_mapper("costProposals")]
    fn cost_proposals(&self) -> SingleValueMapper<BigUint>;
}