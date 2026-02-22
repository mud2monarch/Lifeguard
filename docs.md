- https://docs.propellerheads.xyz/tycho/for-solvers/simulation#installation doesn't work, had to use different toml version to resolve Rust-Analyzer issue "error: failed to load workspaces.". On Cargo version cargo 1.93.1 (083ac5135 2025-12-15).
```toml
[dependencies.tycho-simulation]
git = "https://github.com/propeller-heads/tycho-simulation.git"
package = "tycho-simulation"
tag = "0.243.0"
features = ["evm"]
```
- in the fetch token util section, the example is missing a 'compression' parameter which should probably be true: https://docs.propellerheads.xyz/tycho/for-solvers/simulation#step-1-fetch-tokens. see /Users/zwong/.cargo/git/checkouts/tycho-simulation-0ebd7e601264ca0b/13a6d88/src/utils.rs which is also missing the param in docs.
