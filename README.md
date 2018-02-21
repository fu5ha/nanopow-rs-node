# nanopow-rs-node

A JavaScript wrapper for nanopow-rs to provide fast, safe, fully multithreaded Nano proof-of-work generation in Node.js.

## Usage

### With Node

```
$ yarn add nanopow-rs-node
$ yarn install
```

```javascript
const nanopow = require('nanopow')

let hash = 'AC101449364C84CDD7562AA724BE52757EF06BCE834C50CF610DD2949291B0D9'
const work = nanopow.generateWorkNoLimit(hash)
// or, const work = nanopow.generateWork(hash)

const isValid = nanopow.checkWork(hash, work) // true
```

### In Electron App

Basically same as above, see https://guides.neon-bindings.com/electron-apps/

## API

```javascript
/**
 * Attempts to generate valid work for a given hash. Runs 100,000,000 attempts and if no valid work is found, returns '0000000000000000'.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @return An 8-byte (16-character) hex-encoded string that is the work found. If no valid work was found, returns '0000000000000000'
 */
nanopow.generateWork(hash)

/**
 * Attempts to generate valid work for a given hash. Will continue to run until valid work is found.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @return An 8-byte (16-character) hex-encoded string that is the work found.
 */
nanopow.generateWorkNoLimit(hash)

/**
 * Attempts to generate valid work for a given hash. Will continue to run until valid work is found.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @param work An 8-byte (16-character) hex-encoded string that is the work to be verified.
 * @return Boolean representing whether the given work was valid for the given hash.
 */
nanopow.checkWork(hash, work)
```
