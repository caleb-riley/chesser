-- globals.lua

---@alias PieceColor "white" | "black"

---@class Piece
---@field kind string
---@field color PieceColor
---@field history Move[]
---@field last_moved integer | nil
---@field metadata table

---@class RelocationAction
---@field kind "relocation"
---@field origin Position
---@field destination Position

---@class SpawnAction
---@field kind "spawn"
---@field position Position
---@field id string
---@field color PieceColor

---@class DeletionAction
---@field kind "deletion"
---@field position Position

---@alias Action RelocationAction | SpawnAction | DeletionAction

---@class Move
---@field origin Position
---@field destination Position
---@field actions Action[]
---@field promotions string[]

---@class Board
---@field get_piece fun(self: Board, position: Position): Piece | nil
---@field get_perpendicular_moves fun(self: Board, position: Position): Move[]
---@field get_diagonal_moves fun(self: Board, position: Position): Move[]
---@field get_directional_moves fun(self: Board, position: Position, offsets: Offset[]): Move[]
---@field get_dimensions fun(self: Board): integer
---@field turn_count fun(self: Board): integer

---@alias TerminationState PieceColor | "draw" | nil

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
    return {
        origin = origin,
        destination = destination,
        actions = {
            { kind = "relocation", origin = origin, destination = destination },
        },
        promotions = {},
    }
end

---@param origin Position
---@param destination Position
---@param captures Position[]
---@return Move
function utils.make_capture_move(origin, destination, captures)
    local actions = {}

    for _, capture in ipairs(captures) do
        table.insert(actions, {
            kind = "deletion",
            position = capture,
        })
    end

    table.insert(actions, {
        kind = "relocation",
        origin = origin,
        destination = destination,
    })

    return {
        origin = origin,
        destination = destination,
        actions = actions,
        promotions = {},
    }
end
