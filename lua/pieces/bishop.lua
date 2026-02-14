return {
    value = 3,

    ---@param board Board
    ---@param position Position
    available_moves = function(board, position)
        return board:get_diagonal_moves(position)
    end
}
