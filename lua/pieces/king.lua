local KING_OFFSETS = {
    Offset.new(0, -1),
    Offset.new(-1, -1),
    Offset.new(-1, 0),
    Offset.new(-1, 1),
}

return {
    ---@type integer
    value = 0,

    ---@param board Board
    ---@param piece Piece
    ---@param position Position
    ---@return Move[]
    available_moves = function(board, piece, position)
        return board:get_directional_moves(position, KING_OFFSETS)
    end,
}
