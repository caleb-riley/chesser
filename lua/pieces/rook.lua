return {
    value = 5,

    ---@param board Board
    ---@param position Position
    available_moves = function(board, position)
        return board:get_perpendicular_moves(position)
    end
}
