-- globals.lua

---@class Piece
---@field kind string
---@field color string
---@field position Position
---@field history Move[]
---@field last_moved integer | nil

---@class Move
---@field origin Position
---@field destination Position
---@field kind "passive" | CaptureKind

---@class CaptureKind
---@field type "capture"
---@field captures Position[]

---@class Board
---@field get_piece fun(self: Board, position: Position): Piece | nil
---@field get_perpendicular_moves fun(self: Board, position: Position): Move[]
---@field get_diagonal_moves fun(self: Board, position: Position): Move[]
---@field get_directional_moves fun(self: Board, position: Position, offsets: Offset[]): Move[]
---@field get_dimensions fun(self: Board): integer
---@field turn_count fun(self: Board): integer

---@type table
_G.utils = {}

---Returns a table of moves
---@param position Position
---@return table
function utils.perpendicular_moves(position)
    return {}
end

---Returns a table of moves
---@param position Position
---@return table
function utils.diagonal_moves(position)
    return {}
end

---Returns a table of moves
---@param left table
---@param right table
---@return table
function utils.concat_tables(left, right)
    return {}
end

---@param origin Position
---@param destination Position
---@return Move
function utils.make_passive_move(origin, destination)
    return { origin = origin, destination = destination, kind = "passive" }
end

---@param origin Position
---@param destination Position
---@param captures Position[]
---@return Move
function utils.make_capture_move(origin, destination, captures)
    return {
        origin = origin,
        destination = destination,
        kind = {
            type = "capture",
            captures = captures,
        },
    }
end
