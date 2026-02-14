local KING_OFFSETS = {
    utils.make_offset(0, -1),
    utils.make_offset(-1, -1),
    utils.make_offset(-1, 0),
    utils.make_offset(-1, 1),
}

return {
    value = 0,

    ---@param board Board
    ---@param position Position
    available_moves = function(board, position)
        return board:get_directional_moves(position, KING_OFFSETS)
    end
}
