return {
    value = 1,

    ---@param board Board
    ---@param position Position
    available_moves = function(board, position)
        local piece = board:get_piece(position)

        if piece == nil then
            return {}
        end

        if piece.color == "White" then
            local destination = utils.make_position(position.row - 1, position.column)

            return { utils.make_passive_move(destination) }
        else
            local destination = utils.make_position(position.row + 1, position.column)

            return { utils.make_passive_move(destination) }
        end
    end
}
