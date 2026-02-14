---@class Position
---@field row integer
---@field column integer
---@field offset_by_checked fun(self: Position, offset: Offset, dimensions: integer): Position | nil
---@field offset_by_unchecked fun(self: Position, offset: Offset): Position
---@field offset_to fun(self: Position, position: Position): Offset

Position = {}
Position.__index = Position

---@param row integer
---@param column integer
---@return Position
function Position.new(row, column)
    local self = {
        row = row,
        column = column,
    }

    return setmetatable(self, Position)
end

---@param self Position
---@param offset Offset
---@param dimensions integer
---@return Position | nil
function Position:offset_by_checked(offset, dimensions)
    local position = Position.new(self.row + offset.delta_row, self.column + offset.delta_column)

    if position.row < 0 or position.row >= dimensions or position.column < 0 or position.column >= dimensions then
        return nil
    end

    return position
end

---@param self Position
---@param offset Offset
---@return Position
function Position:offset_by_unchecked(offset)
    return Position.new(self.row + offset.delta_row, self.column + offset.delta_column)
end

---@param self Position
---@param position Position
---@return Offset
function Position:offset_to(position)
    return Offset.new(position.row - self.row, position.column - self.column)
end

---@param self Position
---@param dimensions integer
---@return Position | nil
function Position:restrict_to(dimensions)
    if dimensions >= self.row or dimensions >= self.column then
        return nil
    end

    return Position.new(self.row, self.column)
end

---@class Offset
---@field delta_row integer
---@field delta_column integer
---@field scale_by fun(self: Offset, scale: integer): Offset
---@field offset_by fun(self: Offset, offset: Offset): Offset
---@field taxicab_magnitude fun(self: Offset): integer

Offset = {}
Offset.__index = Offset

---@param delta_row integer
---@param delta_column integer
---@return Offset
function Offset.new(delta_row, delta_column)
    local self = {
        delta_row = delta_row,
        delta_column = delta_column,
    }

    return setmetatable(self, Offset)
end

---@param self Offset
---@param scale integer
---@return Offset
function Offset:scale_by(scale)
    return Offset.new(self.delta_row * scale, self.delta_column * scale)
end

---@param self Offset
---@param offset Offset
---@return Offset
function Offset:offset_by(offset)
    return Offset.new(self.delta_row + offset.delta_row, self.delta_column + offset.delta_column)
end

---@param self Offset
---@return integer
function Offset:taxicab_magnitude()
    return math.abs(self.delta_row) + math.abs(self.delta_column)
end
