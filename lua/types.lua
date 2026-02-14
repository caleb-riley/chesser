-- globals.lua

---@class Position
---@field row number
---@field column number

---@class Offset
---@field delta_row number
---@field delta_column number

---@class Piece
---@field kind string
---@field color string
---@field position Position
---@field last_moved number | nil

---@class Move

---@class Board
---@field get_piece fun(self: Board, position: Position): Piece | nil
---@field get_perpendicular_moves fun(self: Board, position: Position): Move[]
---@field get_diagonal_moves fun(self: Board, position: Position): Move[]
---@field get_directional_moves fun(self: Board, position: Position, offsets: Offset[]): Move[]

---@type table
_G.utils = {}

---Returns a table of moves
---@param row number
---@param column number
---@return Position
function utils.make_position(row, column)
    return {}
end

---Returns a table of moves
---@param delta_row number
---@param delta_column number
---@return Offset
function utils.make_offset(delta_row, delta_column)
    return {}
end

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
