Rust-4MyFuture
==================

Rust Contract of 4MyFutureDApp the repository here https://github.com/NEAR-Hispano/4myFuture
Here you'll find the optimized contract version.


Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`

Exploring The Code
==================

1. The "backend" code lives in the `/contract` folder. See the README there for
   more info.
2. The "scripts" folder will help to automatize some tasks as Deploy, Test and more.


Deploy
======

To deploy the smart contract you will need yo run the following command "scripts/dev-deploy.sh" 

Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli
    npm install --global near-cli
    
Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `Rust-4MyFuture.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `Rust-4MyFuture.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):
      near create-account Rust-4MyFuture.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet


Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages
