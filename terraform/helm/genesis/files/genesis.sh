#!/bin/bash

#
# Runs an automated genesis ceremony for validators spun up by the aptos-node helm chart
#
# Expect the following environment variables to be set before execution:
# WORKSPACE
# NUM_VALIDATORS
# USERNAME_PREFIX
# VALIDATOR_HOST_SUFFIX: default validator-lb
# FULLNODE_HOST_SUFFIX: default fullnode-lb
# 

VALIDATOR_HOST_SUFFIX=${VALIDATOR_HOST_SUFFIX:-validator-lb}
FULLNODE_HOST_SUFFIX=${FULLNODE_HOST_SUFFIX:-fullnode-lb}

# generate all validator configurations
for i in $(seq 0 $(($NUM_VALIDATORS-1))); do
username="${USERNAME_PREFIX}-${i}"
user_dir="${WORKSPACE}/${username}"
mkdir $user_dir
aptos genesis generate-keys --output-dir $user_dir
aptos genesis set-validator-configuration --keys-dir $user_dir --local-repository-dir ${WORKSPACE} \
    --username $username \
    --validator-host "${username}-${VALIDATOR_HOST_SUFFIX}:6180" \
    --full-node-host "${username}-${FULLNODE_HOST_SUFFIX}:6182"
done


# get the framework
# this is the directory the aptos-framework is located in the aptoslabs/init docker image
FRAMEWORK_DIR="/aptos-framework/move/modules"
cp -R $FRAMEWORK_DIR ${WORKSPACE}/framework

# run genesis
aptos genesis generate-genesis --local-repository-dir ${WORKSPACE} --output-dir ${WORKSPACE}

# TODO(rustielin): delete old genesis artifacts

# create genesis for validators to startup
for i in $(seq 0 $(($NUM_VALIDATORS-1))); do
username="${USERNAME_PREFIX}-${i}"
user_dir="${WORKSPACE}/${username}"
kubectl create secret generic {{ $user.name }}-genesis-e{{ $.Values.chain.era }} \
    --from-file=genesis.blob=${WORKSPACE}/genesis.blob \
    --from-file=waypoint.txt=${WORKSPACE}/waypoint.txt \
    --from-file=validator-identity.yaml=${user_dir}/validator-identity.yaml \
    --from-file=validator-full-node-identity.yaml=${user_dir}/validator-full-node-identity.yaml
done
