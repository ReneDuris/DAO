# DAO
This Smart Contract can create Proposal with option voting up to 5 options/single voting(approve,reject). And users can vote on specific proposal with their vote.

#### disclaimer: 
- using this code is on your own risk. This code is meant to be used for inspirational purposes

To interact with the Smart Contract, you would need to have:
- python3.8 and upper
- [Pem wallet](https://github.com/ReneDuris/GeneratePem-erdjs) - Generating pem file through erdjs.
- [Elrond IDE](https://marketplace.visualstudio.com/items?itemName=Elrond.vscode-elrond-ide/) - extension for Visual Studio Code
```
ctrl+shift+P
Elrond: Setup Workspace
```
# Interaction
To interact with the Smart Contract, you would need to supply your wallet with some funds:
- [ESDT/EGLD faucet](https://r3d4.fr/elrond/devnet/)
   
# Contract build
example:
```
erdpy --verbose contract build "/home/project/dao"
```
# Deploying smartcontract through erdpy.json
There is stored whole configuration to get smartcontract deployed.
```
{
    "configurations": {
        "default": {
            "proxy": "https://devnet-api.elrond.com",
            "chainID": "D"
        }
    },
    "contract":{
        "deploy":{
            "verbose": true,
            "bytecode": "output/dao.wasm",
            "recall-nonce": true,
            "pem": "../wallet.pem",
            "gas-limit": 90000000,
            "arguments": [
                "3000",
                "str:DSUPER-9af8df",
                "5000000000000000000",
                "50000000000000000000"
            ],
            "send": true,
            "outfile": "dao.json"
        }
     }
}
```
using this command you will deploy your smartcontract on devnet specified above in "configurations"
```
erdpy contract deploy
```
# Upgrading smartcontract
example:
```
erdpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgq6lsfc55vs5yk56mrnd5s95jgm9lrevvs0ztsrkernq --recall-nonce --pem="wallet.pem" --gas-limit=60000000 --proxy="https://devnet-gateway.elrond.com" --chain=D --project=dao --arguments 3000 str:DSUPER-9af8df 5000000000000000000 50000000000000000000 --send || return
```
# Arguments upon deployement
Upon deployement or while ugrading you have to set your arguments.
- EXPIRATION TIME (How long one proposal will last [seconds])
```
u64 (ex. 3000)
```
- TOKEN ID (Your token id which will be used for payments for creating and voting on proposals. [TokenIdentifier])
```
"str:EgldOrEsdtIdentifier" (ex. str:DSUPER-9af8df)
```
- COST OF VOTE (Cost of voting. [BigUint])
```
BigUint (ex. 5000000000000000000)
```
- COST OF CREATING PROPOSAL (Cost of creation of proposal. [BigUint])
```
BigUint (ex. 50000000000000000000)
```

```
  #[init]
    fn init(&self,
    seconds: u64,
    token_id : EgldOrEsdtTokenIdentifier, 
    cost_vote : BigUint,
    cost_proposals: BigUint) {
    
        self.expiration_time().set(seconds);
        self.token_id().set(token_id);
        self.cost_vote().set(cost_vote);
        self.cost_proposals().set(cost_proposals);
    }
```        

# Query SmartContract
Using view methods you are able to query your smartcontract to view informations.
- [query SmartContract-erdjs](https://github.com/ReneDuris/Query-SmartContract-erdjs)
       
 # View methods, storage mappers
- [View annotations](https://docs.elrond.com/developers/developer-reference/elrond-wasm-annotations/#endpoint-and-view)

Storage mappers can be used to store single values or multiple values. And with view method you are able to query stored value.
- [singleValueMapper](https://docs.elrond.com/developers/developer-reference/storage-mappers/#get)
```
    #[view(getVoteCount)]
    #[storage_mapper("voteCount")]
    fn vote_count(&self,proposal_id: ManagedBuffer,vote:ManagedBuffer) -> SingleValueMapper<usize>;
```
- [UnorderedSetMapper](https://docs.elrond.com/developers/developer-reference/storage-mappers/#unorderedsetmapper)
```
#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode,NestedDecode)]
pub struct ProposalVote<M: ManagedTypeApi> {
    pub user: ManagedAddress<M>,
    pub vote: ManagedBuffer<M>,
    pub vote_time: u64
}

    #[view(getVoteAddress)]
    #[storage_mapper("voteAddress")]
    fn vote_address(&self,proposal_id: ManagedBuffer) -> UnorderedSetMapper<ProposalVote<Self::Api>>;

```      
# Endpoints

### #[only_owner] DAO endpoints

  - changeToken (You are able to change token for payment of voting and creating proposals.)
   ```
    #[only_owner]
    #[endpoint(changeToken)]
    fn change_token(&self,new_token_id : EgldOrEsdtTokenIdentifier){
    }
   ```
  - changeCostVote (Change cost of voting)
   ```
    #[only_owner]
    #[endpoint(changeCostVote)]
    fn change_cost_vote(&self,new_cost_vote : BigUint){
    }
   ```
  - changeCostProposal (Change cost of creating Proposals)
   ```
    #[only_owner]
    #[endpoint(changeCostProposals)]
    fn change_cost_proposals(&self,new_cost_vote : BigUint){
    }
   ```
  - setExpirationTime (Change of expiration time of proposal.
   ```
    #[only_owner]
    #[endpoint(setExpirationTime)]
    fn set_expiration_time(&self,seconds:u64){
    }
   ```
  - proposalPush (Push proposal of specific proposal from user. Automatic when owner creating proposal.)
   ```
     #[only_owner]
     #[endpoint(proposalPush)]
     fn proposal_push(&self,proposal_id:ManagedBuffer){
    }
   ```

### users DAO endpoints

  - createProposal (Creating proposal)
   ```
     #[endpoint(createProposal)]
     fn create_proposal(&self,proposal_content:ManagedBuffer,opt: MultiValueEncoded<ManagedBuffer>)-> SCResult<ManagedBuffer>{
     }
   ```
  - proposalActivate (Activate of proposal, which then need to be pushed by owner to voting)
   ```
     #[payable("*")]
     #[endpoint(proposalActivate)]
     fn proposal_activate(&self,proposal_id:ManagedBuffer){
     }
   ```
  - proposalDelete (Delete of proposal, if proposal is pushed to voting it cannot be deleted)
   ```
     #[endpoint(proposalDelete)]
     fn proposal_delete(&self,proposal_id:ManagedBuffer){
     }
   ```
  - voteProposal (voting to proposal of normal proposal type)
   ```
     #[payable("*")]
     #[endpoint(voteProposal)]
     fn vote_on_proposal(&self,proposal_id:ManagedBuffer, vote: ManagedBuffer){
     }
   ```
 - voteOption (voting on proposal of option proposal type)
  ```
     #[payable("*")]
     #[endpoint(voteOption)]
     fn vote_on_proposal(&self,proposal_id:ManagedBuffer, vote: usize){
     }
   ```

