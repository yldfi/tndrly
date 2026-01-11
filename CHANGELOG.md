# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
