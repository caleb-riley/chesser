-- globals.lua

---@class Position
---@field row number
---@field column number

---@class Piece
---@field kind string
---@field color string
---@field position Position
---@field last_moved number|nil

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
