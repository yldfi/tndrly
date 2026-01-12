# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1](https://github.com/yldfi/tndrly/compare/v0.2.0...v0.2.1) (2026-01-12)


### Bug Fixes

* **api:** correct endpoint paths for alerts and actions APIs ([4bc14b3](https://github.com/yldfi/tndrly/commit/4bc14b3fc14ec7397c0aaab38d53136eb86c6f55))
* **vnets:** use chain_config wrapper in CreateVNetRequest ([dddc5ab](https://github.com/yldfi/tndrly/commit/dddc5abd4a4330a864ca8bb76192d3deed2ea767))

## [0.2.2](https://github.com/yldfi/tndrly/compare/v0.2.1...v0.2.2) (2026-01-12)


### Bug Fixes

* **vnets:** fix delete_many to use correct API endpoint (DELETE /vnets with vnet_ids body)


## [0.2.0](https://github.com/yldfi/tndrly/compare/v0.1.1...v0.2.0) (2026-01-12)


### Features

* **simulation:** add get_full() for complete simulation details ([ccdf254](https://github.com/yldfi/tndrly/commit/ccdf254b16da74dbad923d42582a8ea4596a521c))
* **simulation:** add missing API parameters ([0971749](https://github.com/yldfi/tndrly/commit/097174983061e453a9974ca5eaedc03302af4507)), closes [#5](https://github.com/yldfi/tndrly/issues/5)

## [0.1.1](https://github.com/yldfi/tndrly/compare/v0.1.0...v0.1.1) (2026-01-12)


### Bug Fixes

* improve error handling and add Serialize to response types ([c07bfe2](https://github.com/yldfi/tndrly/commit/c07bfe2a0c69f79d5d6538bca9a3461800fa490f))

## 0.1.0 (2026-01-12)


### Features

* tndrly - Tenderly API client for Rust ([6af59e3](https://github.com/yldfi/tndrly/commit/6af59e39cc664841356ba2e76005a64dacf51dab))


### Bug Fixes

* clippy and formatting issues ([faf9380](https://github.com/yldfi/tndrly/commit/faf9380da55ac9861551be282877b2a5bdf33826))

## [0.1.0] - 2026-01-12

### Added

- **Simulation API**: `simulate()`, `simulate_bundle()`, `list()`, `get()`, `info()`, `share()`, `unshare()`, `trace()`
- **Virtual TestNets API**: `create()`, `list()`, `get()`, `delete()`, `delete_many()`, `fork()`, `update()`, `transactions()`, `get_transaction()`, `send_transaction()`, `simulate()`, `rpc_urls()`
- **Contracts API**: `add()`, `list()`, `get()`, `update()`, `delete()`, `verify()`, `encode_state()`, `add_tag()`, `remove_tag()`, `rename()`, `bulk_tag()`, `delete_tag()`
- **Alerts API**: `create()`, `list()`, `get()`, `update()`, `delete()`, `enable()`, `disable()`, `add_destination()`, `remove_destination()`, `create_webhook()`, `list_webhooks()`, `get_webhook()`, `delete_webhook()`, `test_webhook()`, `history()`, `test_alert()`
- **Actions API**: `create()`, `list()`, `get()`, `update()`, `delete()`, `enable()`, `disable()`, `invoke()`, `logs()`, `get_log()`, `source()`, `update_source()`, `stop()`, `resume()`, `stop_many()`, `resume_many()`, `calls()`, `get_call()`
- **Wallets API**: `list()`, `add()`, `get()`
- **Networks API**: `supported()`, `mainnets()`, `testnets()`, `get()`
- **Delivery Channels API**: `list_project()`, `list_account()`
- Address validation utilities
- Integration test examples
- MIT license
- CI workflow (check, test, fmt, clippy, docs)
- Publish workflow for crates.io releases

[0.1.0]: https://github.com/yldfi/tndrly/releases/tag/v0.1.0
