local KNIGHT_OFFSETS = {
    utils.make_offset(-1, -2),
    utils.make_offset(-2, -1),
    utils.make_offset(-2, 1),
    utils.make_offset(-1, 2),
}

return {
    value = 3,

    ---@param board Board
    ---@param position Position
    available_moves = function(board, position)
        return board:get_directional_moves(position, KNIGHT_OFFSETS)
    end
}
