return {
    ---@type integer
    value = 5,

    ---@param board Board
    ---@param piece Piece
    ---@param position Position
    available_moves = function(board, piece, position)
        return board:get_perpendicular_moves(position)
    end,
}
