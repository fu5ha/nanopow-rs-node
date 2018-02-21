var native = require('../native')

/**
 * Attempts to generate valid work for a given hash.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @param maxIters (optional) The maximum iterations to try before returning. If this parameter is omitted, is null, or is 0, it will run until valid work is found.
 * @return A Promise that is resolved with an 8-byte (16-character) hex-encoded string that is the work found. If no work is found in maxIters, the promise is rejected.
 */
function generateWork (hash, maxIters) {
  if (maxIters === undefined || maxIters === null) {
    maxIters = 0
  }
  return new Promise((resolve, reject) => {
    native.generateWork(hash, maxIters, (err, work) => {
      if (err) {
        reject(err)
      } else {
        resolve(work)
      }
    })
  })
}

/**
 * Attempts to generate valid work for a given hash.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @param maxIters (optional) The maximum iterations to try before returning. If this parameter is omitted, is null, or is 0, it will run until valid work is found.
 * @return An 8-byte (16-character) hex-encoded string that is the work found. If no valid work was found in maxIters, returns '0000000000000000'
 */
function generateWorkSync (hash, maxIters) {
  if (maxIters === undefined || maxIters === null) {
    maxIters = 0
  }
  return native.generateWorkSync(hash, maxIters)
}

/**
 * Checks whether given work is valid for a given hash.
 * @param hash A 32-byte (64-character) hex-encoded string. This will either be the previous block hash, or, if there is no previous block, the account's public key.
 * @param work An 8-byte (16-character) hex-encoded string that is the work to be verified.
 * @return Boolean representing whether the given work was valid for the given hash.
 */
function checkWork (hash, work) {
  return native.checkWork(hash, work)
}

module.exports = {
  generateWork: generateWork,
  generateWorkSync: generateWorkSync,
  checkWork: checkWork
}
