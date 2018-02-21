/* global describe it */
const assert = require('assert')
const nanopow = require('../.')

describe('nanopow', () => {
  describe('#checkWork()', () => {
    it('should validate good work', () => {
      const hash = '8D3E5F07BFF7B7484CDCB392F47009F62997253D28BD98B94BCED95F03C4DA09'
      const work = '4effb6b0cd5625e2'
      const valid = nanopow.checkWork(hash, work)
      assert(valid)
    })
    it('should reject bad work', () => {
      const hash = '8D3E5F07BFF7B7484CDCB392F47009F62997253D28BD98B94BCED95F03C4DA09'
      const work = '5effb6b0cd5625e2'
      const valid = nanopow.checkWork(hash, work)
      assert(!valid)
    })
  })
  describe('#generateWork()', () => {
    it('should generate valid work', () => {
      const hash = '8D3E5F07BFF7B7484CDCB392F47009F62997253D28BD98B94BCED95F03C4DA09'
      const work = nanopow.generateWork(hash)
      const valid = nanopow.checkWork(hash, work)
      assert(valid)
    })
  })
})
