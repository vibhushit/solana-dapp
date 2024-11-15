const assert = require('assert')
const anchor = require('@project-serum/anchor')
const {SystemProgrm} = anchor.web3

describe('dapp', () => {

  const provider = anchor.Provider.local();
  anchor.setProvider(provider)

  let calculator = anchor.web3.Keypair.generate()
  const program = calculator.workspace.Dapp

  it('Creates a calculator', async() => {
    await program.rpc.create("Welcome to Solana", {
      accounts: {
        
      }
    })
  })
})