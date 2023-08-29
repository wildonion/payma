

# pip install thirdweb-sdk
# https://thirdweb.com/dashboard/contracts

from thirdweb import ThirdwebSDK
from thirdweb.types.nft import NFTMetadataInput
from thirdweb.types import SDKOptions
from fastapi import FastAPI
from thirdweb.types import ContractPlatformFeeSchema

app = FastAPI()



@app.post("/deposit/{amount}")
def deposit(mint_to, amount):
    
    thirdweb_secret_key = "BPCpEo42xF34w5o2fdJ6uqv0I_m4jwDGHUzu05pNDd7EBZ9iUW3ULt4tgNqvKZE5WGQR8r42X4CofwDcAd6uzg"
    wallet_private_key = "5fcec999332133dcbca2ed18b83f87669087b9dd9f86691bca68a252aae3da02"
    split_contract = ""
    sdk = ThirdwebSDK.from_private_key(wallet_private_key, "mumbai", SDKOptions(secret_key=thirdweb_secret_key))
    split_contract_handler = sdk.get_contract(split_contract)



@app.post("/withdraw/{amount}")
def withdraw(token_id):

    thirdweb_secret_key = "BPCpEo42xF34w5o2fdJ6uqv0I_m4jwDGHUzu05pNDd7EBZ9iUW3ULt4tgNqvKZE5WGQR8r42X4CofwDcAd6uzg"
    wallet_private_key = "5fcec999332133dcbca2ed18b83f87669087b9dd9f86691bca68a252aae3da02"
    split_contract = ""
    sdk = ThirdwebSDK.from_private_key(wallet_private_key, "mumbai", SDKOptions(secret_key=thirdweb_secret_key))
    split_contract_handler = sdk.get_contract(split_contract)
