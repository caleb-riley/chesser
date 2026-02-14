return {
    ---@type integer
    value = 3,

    ---@param board Board
    ---@param piece Piece
    ---@param position Position
    available_moves = function(board, piece, position)
        return board:get_diagonal_moves(position)
    end,
}
