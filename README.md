# nanopow-rs-node

A JavaScript wrapper for nanopow-rs to provide fast, safe, fully multithreaded Nano proof-of-work generation in Node.js.

## Usage

### First: Install Rust

See https://www.rustup.rs/ for instructions on installing Rust for your system.

### With Node

```
$ yarn add nanopow-rs-node
$ yarn install
```

### In Electron App

Basically same as above, see https://guides.neon-bindings.com/electron-apps/

## Example

```javascript
// Require as usual
const nanopow = require('nanopow-rs-node')

// The hash is either the hash of the previous block in the account's chain
// or the account's public key if there are no previous blocks.
const hash = 'AC101449364C84CDD7562AA724BE52757EF06BCE834C50CF610DD2949291B0D9'

// Asynchronously (non-blocking) with no limit
nanopow.generateWork(hash)
  .then(work => {
    const isValid = nanopow.checkWork(hash, work) // true
  })

// Asynchronously (non-blocking) with limit
nanopow.generateWork(hash, 10000000) // 10 million
  .then(work => {
    const isValid = nanopow.checkWork(hash, work) // true
    console.log(`Found valid work: ${work}`)
  })
  .catch(err => {
    console.error(`Failed to find work in 10 million iterations`)
  })

// Asynchronously using async/await with no limit
async function myTransactionFunction (hash) {
  const work = await nanopow.generateWork(hash)
  const isValid = nanopow.checkWork(hash, work) // true
}

// Asynchronously using async/await with limit
async function myTransactionFunction (hash) {
  try {
    const work = await nanopow.generateWork(hash, 10000000) // 10 million
    const isValid = nanopow.checkWork(hash, work) // true
    console.log(`Found valid work: ${work}`)
  } catch (e) {
    console.error(`Failed to find work in 10 million iterations`)
  }
}

// Synchronously (blocking) no limit
const work = nanopow.generateWorkSync(hash) // no limit
const isValid = nanopow.checkWork(hash, work) // true

// Synchronously (blocking) no limit
const work = nanopow.generateWorkSync(hash, 10000000) // 10 million
const isValid = nanopow.checkWork(hash, work) // Could be false if valid work was not found
```

## API Reference

```javascript
/**
 * Attempts to generate valid work for a given hash.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @param maxIters (optional) The maximum iterations to try before returning. If this parameter is omitted, is null, or is 0, it will run until valid work is found.
 * @return A Promise that is resolved with an 8-byte (16-character) hex-encoded string that is the work found. If no work is found in maxIters, the promise is rejected.
 */
nanopow.generateWork (hash, maxIters)

/**
 * Attempts to generate valid work for a given hash.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @param maxIters (optional) The maximum iterations to try before returning. If this parameter is omitted, is null, or is 0, it will run until valid work is found.
 * @return An 8-byte (16-character) hex-encoded string that is the work found. If no valid work was found in maxIters, returns '0000000000000000'
 */
nanopow.generateWorkSync (hash, maxIters)

/**
 * Checks whether given work is valid for a given hash.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @param work An 8-byte (16-character) hex-encoded string that is the work to be verified.
 * @return Boolean representing whether the given work was valid for the given hash.
 */
nanopow.checkWork(hash, work)
```

## Development

Install needed deps with
```
$ yarn install
```

Tests in `test/test.js`, perfom tests with
```
$ yarn test
```

### Reference

See Neon [docs](https://api.neon-bindings.com/neon/index.html) and [examples in their tests directory on GitHub](https://github.com/neon-bindings/neon/tree/master/test/dynamic)
