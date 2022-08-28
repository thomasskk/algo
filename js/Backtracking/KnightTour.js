// Wikipedia: https://en.wikipedia.org/wiki/Knight%27s_tour

export const OpenKnightTour = (size) => {
  return {
    board: new Array(size).fill(0).map(() => new Array(size).fill(0)),
    getMoves([i, j]) {
      // helper function to get the valid moves of the knight from the current position
      const moves = [
        [i + 2, j - 1],
        [i + 2, j + 1],
        [i - 2, j - 1],
        [i - 2, j + 1],
        [i + 1, j - 2],
        [i + 1, j + 2],
        [i - 1, j - 2],
        [i - 1, j + 2],
      ]

      return moves.filter(([y, x]) => y >= 0 && y < size && x >= 0 && x < size)
    },

    isComplete() {
      // helper function to check if the board is complete
      return !this.board.map((row) => row.includes(0)).includes(true)
    },

    solve() {
      // function to find the solution for the given board
      for (let i = 0; i < size; i++) {
        for (let j = 0; j < size; j++) {
          if (this.solveHelper([i, j], 0)) {
            return true
          }
        }
      }
      return false
    },

    solveHelper([i, j], curr) {
      // helper function for the main computation
      if (this.isComplete()) {
        return true
      }

      for (const [y, x] of this.getMoves([i, j])) {
        if (this.board[y][x] === 0) {
          this.board[y][x] = curr + 1
          if (this.solveHelper([y, x], curr + 1)) {
            return true
          }
          // backtracking
          this.board[y][x] = 0
        }
      }
      return false
    },
  }
}
