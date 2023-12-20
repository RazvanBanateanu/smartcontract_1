JIM="../../wallet/jim.pem"
ADDRESS="erd1qqqqqqqqqqqqqpgqeh9jwudt0acyccv6g48t2srjw3rnfk5v67jsl0tsv2"
PROXY=https://testnet-api.multiversx.com

add() {
    read -p "Enter number: " NUMBER
    mxpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=5000000 --function="add" --arguments ${NUMBER} --send --proxy=${PROXY} --chain=T
}

getSum() {
    mxpy --verbose contract query ${ADDRESS} --function="getSum" --proxy=${PROXY}
}
