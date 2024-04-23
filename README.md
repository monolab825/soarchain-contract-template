# Soarchain Starter Pack

This is a template to build smart contracts in Rust to run inside a [Soarchain Node].

To understand the framework better, please read the overview in the
[Soarchain Documentation](https://docs.soarchain.com/category/smart-contracts/),
and dig into the [template smart contracts docs](https://docs.soarchain.com/category/smart-contracts/templates).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

Assuming you have a recent version of Rust and Cargo installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

**Latest**

```sh
cargo generate --git https://github.com/soar-robotics/soarchain-contract-template.git --name PROJECT_NAME
```

For cloning minimal code repo:

```sh
cargo generate --git https://github.com/soar-robotics/soarchain-contract-template.git --name PROJECT_NAME -d minimal=true
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.

## Create a Repo

After generating, you have a initialized local git repo, but no commits, and no remote.
Go to a server (eg. github) and create a new upstream repo (called `YOUR-GIT-URL` below).
Then run the following:

```sh
# this is needed to create a valid Cargo.lock file (see below)
cargo check
git branch -M main
git add .
git commit -m 'Initial Commit'
git remote add origin YOUR-GIT-URL
git push -u origin main
```

## Using your project

Once you have your custom repo, you should check out [Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.soarchain.com/category/smart-contracts/templates/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And

Please replace this README file with information about your specific project. You can keep
the `Developing.md` and `Publishing.md` files as useful references, but please set some
proper description in the README.
