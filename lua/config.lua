local BACK_ROWS = { "rook", "knight", "bishop", "queen", "king", "bishop", "knight", "rook" }
local EMPTY = {}

---@class InitialPiece
---@field id string
---@field color PieceColor

---@return InitialPiece[][]
local function get_default_board()
    local initial_layout = { {}, {}, {}, {}, {}, {}, {}, {} }

    for _, id in ipairs(BACK_ROWS) do
        table.insert(initial_layout[1], { id = id, color = "black" })
    end

    for _ = 1, 8 do
        table.insert(initial_layout[2], { id = "pawn", color = "black" })
    end

    for column = 3, 6 do
        for _ = 1, 8 do
            table.insert(initial_layout[column], EMPTY)
        end
    end

    for _ = 1, 8 do
        table.insert(initial_layout[7], { id = "pawn", color = "white" })
    end

    for _, id in ipairs(BACK_ROWS) do
        table.insert(initial_layout[8], { id = id, color = "white" })
    end

    return initial_layout
end

return {
    ---@type InitialPiece[][]
    initial_layout = get_default_board(),

    ---@param board Board
    ---@return TerminationState
    check_termination = function(board)
        local white_king, black_king = false, false

        for row = 0, 7 do
            for column = 0, 7 do
                local piece = board:get_piece(Position.new(row, column))

                if piece ~= nil and piece.kind == "king" then
                    if piece.color == "white" then
                        white_king = true
                    elseif piece.color == "black" then
                        black_king = true
                    end
                end
            end
        end

        if not white_king and not black_king then
            return "draw"
        elseif not white_king then
            return "black"
        elseif not black_king then
            return "white"
        end

        return nil
    end
}
