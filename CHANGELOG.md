# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

## [0.2.0](https://github.com/tirithen/tradfri_gateway/compare/v0.1.0...v0.2.0) (2023-11-18)


### ⚠ BREAKING CHANGES

* Removes type state pattern from TradfriGateway,
Device, and Light. Renames several struct/enum types, and removes the
TradfriGatewayConnector.

### Features

* add groups ([917df10](https://github.com/tirithen/tradfri_gateway/commit/917df10ea98849381c76df8de15d45d995235708))


### Bug Fixes

* one put call per connection, simplify types ([27baba1](https://github.com/tirithen/tradfri_gateway/commit/27baba1be6c41b5b362b8bba863387d3cd72c02c))

## 0.1.0 (2023-11-16)


### Features

* add mDNS to auto discover gateway ip ([01b4ec6](https://github.com/tirithen/tradfri_gateway/commit/01b4ec6f664e4298529c493e732ef64efb344209))
* setup connection and basic light on/off ([bca21a3](https://github.com/tirithen/tradfri_gateway/commit/bca21a324c9e38934632ff11a6c20687668ec81c))
