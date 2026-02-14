return {
    ---@type integer
    value = 1,

    ---@param board Board
    ---@param piece Piece
    ---@param position Position
    ---@return Move[]
    available_moves = function(board, piece, position)
        local moves = {}

        local delta_row = piece.color == "White" and -1 or 1

        local forward_one = position:offset_by_checked(Offset.new(delta_row, 0), board:get_dimensions())

        if forward_one ~= nil and board:get_piece(forward_one) == nil then
            table.insert(moves, utils.make_passive_move(position, forward_one))
        end

        if piece.last_moved == nil then
            local forward_two = position:offset_by_checked(Offset.new(delta_row * 2, 0), board:get_dimensions())

            if forward_two ~= nil and board:get_piece(forward_two) == nil then
                table.insert(moves, utils.make_passive_move(position, forward_two))
            end
        end

        for delta_column = -1, 1, 2 do
            local diagonal_position = position:offset_by_checked(
                Offset.new(delta_row, delta_column), board:get_dimensions())

            if diagonal_position ~= nil then
                local diagonal_target = board:get_piece(diagonal_position)

                if diagonal_target ~= nil and diagonal_target.color ~= piece.color then
                    table.insert(moves, utils.make_capture_move(position, diagonal_position, { diagonal_position }))
                end

                local lateral_position = position:offset_by_checked(Offset.new(0, delta_column), board:get_dimensions())

                if lateral_position ~= nil then
                    local target = board:get_piece(lateral_position)

                    if target ~= nil and target.color ~= piece.color and #target.history > 0 then
                        local previous = target.history[#target.history]

                        if previous.origin:offset_to(lateral_position):taxicab_magnitude() == 2
                            and target.last_moved == board:turn_count() then
                            table.insert(moves,
                                utils.make_capture_move(position, diagonal_position, { lateral_position }))
                        end
                    end
                end
            end
        end

        return moves
    end,
}
