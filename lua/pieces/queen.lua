return {
    ---@type integer
    value = 9,

    ---@param board Board
    ---@param position Position
    available_moves = function(board, position)
        local diagonal_moves = board:get_diagonal_moves(position)
        local perpendicular_moves = board:get_perpendicular_moves(position)

        return utils.concat_tables(diagonal_moves, perpendicular_moves)
    end
}
