# Rust_mint
# Solana Smart Contract Deployment and Testing Guide

This guide explains how to deploy and test the smart contract on a local Solana testnet.

## Prerequisites

Ensure you have the following installed on your system:
1. **Rust and Cargo**
2. **Anchor CLI**
3. **Solana CLI**

## Setup

### Start a Local Solana Test Validator

Open a terminal and start the Solana test validator:
solana-test-validator


In a new terminal window, configure Solana to use the local testnet using the line
solana config set --url http://127.0.0.1:8899

**Build the smart contract using Anchor:**
anchor build


**Deploy the smart contract to the local testnet:**
anchor deploy


**Run the tests to ensure the smart contract works correctly:**
anchor test


