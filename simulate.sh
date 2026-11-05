#!/bin/bash

CONFIG="$1"
OUTPUT_DIR="$2"
EXEC_PATH="$3"

$EXEC_PATH/rhex-craft genesis -c $CONFIG
$EXEC_PATH/rhex-craft submit --rhex $OUTPUT_DIR/genesis.rhex --request submit
$EXEC_PATH/rhex-craft submit --rhex $OUTPUT_DIR/first_policy.rhex --request submit
$EXEC_PATH/rhex-craft submit --rhex $OUTPUT_DIR/manifesto_0.rhex --request submit
$EXEC_PATH/rhex-craft submit --rhex $OUTPUT_DIR/key_grant.rhex --request submit