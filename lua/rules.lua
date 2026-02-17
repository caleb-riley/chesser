return {
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
