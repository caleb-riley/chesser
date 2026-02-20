return {
    ---@param id string
    ---@return nil
    on_piece_relocated = function(id)
        utils.debug("Piece relocated: " .. id)
    end,

    ---@param id string
    ---@return nil
    on_piece_spawned = function(id)
        utils.debug("Piece spawned: " .. id)
    end,

    ---@param id string
    ---@return nil
    on_piece_deleted = function(id)
        utils.debug("Piece deleted: " .. id)
    end,

    ---@param color PieceColor
    ---@return nil
    on_turn_started = function(color)
        utils.debug("Turn started for " .. color)
    end,

    ---@param color PieceColor
    ---@return nil
    on_turn_ended = function(color)
        utils.debug("Turn ended for " .. color)
    end,

    ---@param piece Piece
    ---@param move Move
    ---@return boolean
    validate_move = function(piece, move)
        utils.debug("Validating move for " .. piece.id)

        return true
    end,
}
