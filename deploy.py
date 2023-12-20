from pathlib import Path

from multiversx_sdk_core import CodeMetadata, Address
from multiversx_sdk_core.transaction_builders import ContractDeploymentBuilder
from multiversx_sdk_wallet import UserSigner
from multiversx_sdk_core.transaction_builders import DefaultTransactionBuildersConfiguration
from multiversx_sdk_network_providers import ApiNetworkProvider

config = DefaultTransactionBuildersConfiguration(chain_id="D")

metadata = CodeMetadata(upgradeable=True, readable=True, payable=True, payable_by_contract=True)

signer = UserSigner.from_pem_file(Path("../wallet/jim.pem"))

jim = Address.from_bech32("erd13s6ls03ef0qz7y20cg46j6e30g89pm72vusu5k5skch3l54r67jstk84dc")

builder = ContractDeploymentBuilder(
    config,
    owner=jim,
    deploy_arguments=[],
    code_metadata=metadata,
    code=Path("./output/mycounter.wasm").read_bytes(),
    gas_limit=50000000
)

tx = builder.build()
print("Transaction:", tx.to_dictionary())
print("Transaction data:", tx.data)

provider = ApiNetworkProvider("https://devnet-api.multiversx.com")
jim_on_network = provider.get_account(jim)

tx.nonce = jim_on_network.nonce


tx.signature = signer.sign(tx)

hash = provider.send_transaction(tx)
print("Transaction hash:", hash)