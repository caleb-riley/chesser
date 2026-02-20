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
        local moves = {}

        for _, offset in ipairs(KING_OFFSETS) do
            for _, scale in ipairs({ -1, 1 }) do
                local destination = position:offset_by_checked(offset:scale_by(scale), board:get_dimensions())

                if destination ~= nil then
                    local target = board:get_piece_at_position(destination)

                    if target == nil then
                        table.insert(moves, utils.make_passive_move(position, destination))
                    elseif target.color ~= piece.color then
                        table.insert(moves, utils.make_capture_move(position, destination, { destination }))
                    end
                end
            end
        end

        -- Castling logic
        -- if piece.last_moved == nil and not board:is_in_check(piece.color) then
        if piece.last_moved == nil then
            for _, delta_column in ipairs({ -1, 1 }) do
                local rook_column = delta_column == 1 and board:get_dimensions() - 1 or 0
                local rook_position = Position.new(position.row, rook_column)
                local rook = board:get_piece_at_position(rook_position)

                if rook ~= nil and rook.kind == "rook" and rook.color == piece.color and rook.last_moved == nil then
                    -- Check that squares between king and rook are empty
                    local blocked = false

                    local start_col = math.min(position.column, rook_column) + 1
                    local end_col = math.max(position.column, rook_column) - 1

                    for c = start_col, end_col do
                        if board:get_piece_at_position(Position.new(position.row, c)) ~= nil then
                            blocked = true
                            break
                        end
                    end

                    if not blocked then
                        -- Check that king does not pass through or end in check
                        -- local safe = true
                        -- for c = position.column + direction, position.column + direction * 2, direction do
                        --     local pos = Position.new(row, c)
                        --     if board:would_be_in_check(piece.color, pos) then
                        --         safe = false
                        --         break
                        --     end
                        -- end

                        -- if safe then
                        local castling_destination = Position.new(position.row, position.column + 2 * delta_column)
                        local new_rook_position = castling_destination:offset_by_unchecked(Offset.new(0, -delta_column))

                        local move = utils.make_passive_move(position, castling_destination)
                        table.insert(move.actions,
                            { kind = "relocation", origin = rook_position, destination = new_rook_position })

                        table.insert(moves, move)
                        -- end
                    end
                end
            end
        end

        return moves
    end,
}
