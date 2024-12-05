const test_input = @embedFile("test-input.txt");
const input = @embedFile("input.txt");
const std = @import("std");

const Direction = enum {
    all,
    up,
    down,
    left,
    right,
    up_left,
    up_right,
    down_left,
    down_right,
};

fn mapToken(token: u8) u8 {
    switch (token) {
        0 => return 'X',
        1 => return 'M',
        2 => return 'A',
        3 => return 'S',
        else => @panic("unreachable"),
    }
}

fn isTopEdge(_: []const []const u8, row: usize, _: usize) bool {
    return row == 0;
}

fn isBottomEdge(matrix: []const []const u8, row: usize, _: usize) bool {
    return row == matrix.len - 1;
}

fn isLeftEdge(_: []const []const u8, _: usize, col: usize) bool {
    return col == 0;
}

fn isRightEdge(matrix: []const []const u8, row: usize, col: usize) bool {
    return col == matrix[row].len - 1;
}

fn find(matrix: []const []const u8, row: usize, col: usize, token: u8, direction: Direction) u32 {
    if (matrix[row][col] != mapToken(token)) return 0;
    if (token == 3) return 1;

    var counter: u32 = 0;

    // Go up
    if (!isTopEdge(matrix, row, col) and
        (direction == .all or direction == .up))
    {
        counter += find(matrix, row - 1, col, token + 1, direction);
    }

    // Go up-right
    if (!isTopEdge(matrix, row, col) and
        !isRightEdge(matrix, row, col) and
        (direction == .all or direction == .up_right))
    {
        counter += find(matrix, row - 1, col + 1, token + 1, direction);
    }

    // Go right
    if (!isRightEdge(matrix, row, col) and
        (direction == .all or direction == .right))
    {
        counter += find(matrix, row, col + 1, token + 1, direction);
    }

    // Go down-right
    if (!isBottomEdge(matrix, row, col) and
        !isRightEdge(matrix, row, col) and
        (direction == .all or direction == .down_right))
    {
        counter += find(matrix, row + 1, col + 1, token + 1, direction);
    }

    // Go down
    if (!isBottomEdge(matrix, row, col) and
        (direction == .all or direction == .down))
    {
        counter += find(matrix, row + 1, col, token + 1, direction);
    }

    // Go down-left
    if (!isBottomEdge(matrix, row, col) and
        !isLeftEdge(matrix, row, col) and
        (direction == .all or direction == .down_left))
    {
        counter += find(matrix, row + 1, col - 1, token + 1, direction);
    }

    // Go left
    if (!isLeftEdge(matrix, row, col) and
        (direction == .all or direction == .left))
    {
        counter += find(matrix, row, col - 1, token + 1, direction);
    }

    // Go up-left
    if (!isTopEdge(matrix, row, col) and
        !isLeftEdge(matrix, row, col) and
        (direction == .all or direction == .up_left))
    {
        counter += find(matrix, row - 1, col - 1, token + 1, direction);
    }

    return counter;
}

fn partOne(matrix: []const []const u8) u32 {
    var counter: u32 = 0;
    for (0..matrix.len) |row| {
        for (0..matrix[row].len) |col| {
            counter += find(matrix, row, col, 0, .up);
            counter += find(matrix, row, col, 0, .down);
            counter += find(matrix, row, col, 0, .left);
            counter += find(matrix, row, col, 0, .right);
            counter += find(matrix, row, col, 0, .up_left);
            counter += find(matrix, row, col, 0, .up_right);
            counter += find(matrix, row, col, 0, .down_left);
            counter += find(matrix, row, col, 0, .down_right);
        }
    }
    return counter;
}

fn partTwo(matrix: []const []const u8) u32 {
    var counter: u32 = 0;
    for (0..matrix.len) |row| {
        for (0..matrix[row].len) |col| {
            if (matrix[row][col] != 'A') continue;
            if (isTopEdge(matrix, row, col) or
                isBottomEdge(matrix, row, col) or
                isLeftEdge(matrix, row, col) or
                isRightEdge(matrix, row, col))
            {
                continue;
            }

            if (((matrix[row - 1][col - 1] == 'S' and (matrix[row + 1][col + 1] == 'M')) or
                (matrix[row - 1][col - 1] == 'M' and (matrix[row + 1][col + 1] == 'S'))) and
                ((matrix[row + 1][col - 1] == 'S' and (matrix[row - 1][col + 1] == 'M')) or
                (matrix[row + 1][col - 1] == 'M' and (matrix[row - 1][col + 1] == 'S'))))
            {
                counter += 1;
            }
        }
    }
    return counter;
}

fn parseInput(allocator: std.mem.Allocator, buffer: []const u8) ![][]u8 {
    var arr = std.ArrayList([]u8).init(allocator);
    var lineIter = std.mem.tokenize(u8, buffer, "\n");
    while (lineIter.next()) |line| {
        const line_ = try allocator.dupe(u8, line);
        try arr.append(line_);
    }
    return arr.toOwnedSlice();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    const matrix = try parseInput(allocator, input);
    defer allocator.free(matrix);
    defer for (matrix) |row| allocator.free(row);
    const part_one = partOne(matrix);
    const part_two = partTwo(matrix);
    std.debug.print("Part one: {}\n", .{part_one});
    std.debug.print("Part two: {}\n", .{part_two});
}

test "part-one" {
    const allocator = std.testing.allocator;
    const matrix = try parseInput(allocator, test_input);
    defer allocator.free(matrix);
    defer for (matrix) |row| allocator.free(row);
    const part_one = partOne(matrix);
    try std.testing.expectEqual(18, part_one);
}

test "part-two" {
    const allocator = std.testing.allocator;
    const matrix = try parseInput(allocator, test_input);
    defer allocator.free(matrix);
    defer for (matrix) |row| allocator.free(row);
    const part_two = partTwo(matrix);
    try std.testing.expectEqual(9, part_two);
}
