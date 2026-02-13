return {
    value = 1,
    available_moves = function(board, position)
        -- utils.diagonal_moves(position)
        -- print(board:get_piece({ row = 0, col = 4 }).kind)

        local moves = {}

        for column = 1, 8 do
            local target = utils.make_position(0, column - 1)

            table.insert(moves, utils.make_capture_move(target, { target }))
        end

        return moves
    end
}
