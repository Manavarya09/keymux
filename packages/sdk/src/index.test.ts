import { describe, it, expect } from 'vitest'
import { greet } from './index'

describe('greet', () => {
  it('says hello', () => {
    expect(greet('world')).toBe('hello world')
  })
})
