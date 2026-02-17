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
                    local target = board:get_piece(destination)

                    if target == nil then
                        table.insert(moves, utils.make_passive_move(position, destination))
                    elseif target.color ~= piece.color then
                        table.insert(moves, utils.make_capture_move(position, destination, { destination }))
                    end
                end
            end
        end

        return moves
    end,
}
