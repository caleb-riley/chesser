local KNIGHT_OFFSETS = {
    Offset.new(-1, -2),
    Offset.new(-2, -1),
    Offset.new(-2, 1),
    Offset.new(-1, 2),
}

return {
    ---@type integer
    value = 3,

    ---@param board Board
    ---@param piece Piece
    ---@param position Position
    available_moves = function(board, piece, position)
        return board:get_directional_moves(position, KNIGHT_OFFSETS)
    end,
}
