return {
    ---@type integer
    value = 1,

    ---@param board Board
    ---@param position Position
    available_moves = function(board, position)
        local piece = board:get_piece(position)

        if piece == nil then
            return {}
        end

        local moves = {}

        local delta_row = ({ [true] = -1, [false] = 1 })[piece.color == "White"]

        local forward_one = utils.make_position(position.row + delta_row, position.column)

        if board:in_bounds(position) and board:get_piece(forward_one) == nil then
            table.insert(moves, { destination = forward_one, kind = "passive" })
        end

        if piece.last_moved == nil then
            local forward_two = utils.make_position(position.row + delta_row * 2, position.column)

            if board:in_bounds(forward_two) and board:get_piece(forward_two) == nil then
                table.insert(moves, { destination = forward_two, kind = "passive" })
            end
        end

        for delta_column = -1, 1, 2 do
            local diagonal_position = utils.make_position(position.row + delta_row, position.column + delta_column)

            if board:in_bounds(diagonal_position) then
                local target = board:get_piece(diagonal_position)

                if target ~= nil and target.color ~= piece.color then
                    table.insert(moves, {
                        destination = diagonal_position,
                        kind = {
                            type = "capture",
                            captures = { diagonal_position },
                        },
                    })
                end

                local lateral_position = utils.make_position(position.row, position.column + delta_column)

                if board:in_bounds(position) then
                    local target = board:get_piece(lateral_position)

                    if target ~= nil and target.color ~= piece.color then -- also check distance and turn count later on
                        table.insert(moves, {
                            destination = diagonal_position,
                            kind = {
                                type = "capture",
                                captures = { lateral_position },
                            },
                        })
                    end
                end
            end
        end

        return moves
    end
}
