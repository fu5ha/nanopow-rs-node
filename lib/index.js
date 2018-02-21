var native = require('../native')

/**
 * Attempts to generate valid work for a given hash. Runs 100,000,000 attempts and if no valid work is found, returns '0000000000000000'.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @return An 8-byte (16-character) hex-encoded string that is the work found. If no valid work was found, returns '0000000000000000'
 */
function generateWork (hash) {
  return native.generateWork(hash)
}

/**
 * Attempts to generate valid work for a given hash. Will continue to run until valid work is found.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @return An 8-byte (16-character) hex-encoded string that is the work found.
 */
function generateWorkNoLimit (hash) {
  return native.generateWorkNoLimit(hash)
}

/**
 * Attempts to generate valid work for a given hash. Will continue to run until valid work is found.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @param work An 8-byte (16-character) hex-encoded string that is the work to be verified.
 * @return Boolean representing whether the given work was valid for the given hash.
 */
function checkWork (hash, work) {
  return native.checkWork(hash, work)
}

module.exports = {
  generateWork: generateWork,
  generateWorkNoLimit: generateWorkNoLimit,
  checkWork: checkWork
}
