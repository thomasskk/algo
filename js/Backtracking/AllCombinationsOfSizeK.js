/*
Given two numbers, n and k, make all unique combinations of k numbers from 1 to n and in sorted order
*/

export const combine = (n, k) => {
  let combinations = []
  let current = []

  return {
    findCombination(start = 1) {
      if (current.length == k) {
        combinations.push([...current])
      } else {
        for (let i = start; i <= n; i++) {
          current.push(i)
          this.findCombination(i + 1)
          current.pop()
        }
      }
      return combinations
    },
  }
}
