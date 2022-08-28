/*
 * Problem Statement: Generate all distinct permutations of a an array (all permutations should be in sorted order);
 *
 * What is permutations?
 * - Permutation means possible arrangements in a set (here it is an array);
 *
 * Reference to know more about permutations:
 * - https://www.britannica.com/science/permutation
 *
 */

const swap = (arr, i, j) => {
  const newArray = [...arr]

  ;[newArray[i], newArray[j]] = [newArray[j], newArray[i]]

  return newArray
}

const permutations = (arr) => {
  const P = []
  const high = arr.length - 1

  const permute = (low, arr) => {
    if (low === high) {
      P.push([...arr])
      return P
    }

    for (let i = low; i <= high; i++) {
      arr = swap(arr, low, i)
      permute(low + 1, arr)
    }

    return P
  }
  return permute(0, arr)
}

export { permutations }
