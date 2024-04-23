#!/bin/sh

sudo docker kill soarchain-contract-template-validator-1 && sudo docker rm soarchain-contract-template-validator-1

echo '@@@ soarchain-contract-template-validator-1 killed and removed'