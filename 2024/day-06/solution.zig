const test_input = @embedFile("test-input.txt");
const input = @embedFile("input.txt");
const std = @import("std");
const set = @import("ziglangSet");

const GuardDirection = union(enum) {
    none,
    up,
    right,
    down,
    left,
};

const Guard = struct {
    position: [2]usize,
    direction: GuardDirection,
};

fn isGuard(char: u8) GuardDirection {
    if (char == '^') return .up;
    if (char == '>') return .right;
    if (char == 'v') return .down;
    if (char == '<') return .left;
    return .none;
}

fn findGuard(map: []const []const u8) Guard {
    for (0..map.len) |row| {
        for (0..map[row].len) |col| {
            const direction = isGuard(map[row][col]);
            if (direction == .none) continue;
            return Guard{ .position = [2]usize{ row, col }, .direction = direction };
        }
    }
    @panic("unreachable");
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

fn partOne(allocator: std.mem.Allocator, map: []const []const u8, guard: *Guard) !u32 {
    var counter: u32 = 0;
    var visited = set.HashSetManaged([2]usize).init(allocator);
    defer visited.deinit();
    while (true) {
        const row = guard.position[0];
        const col = guard.position[1];
        const added = try visited.add(guard.position);
        if (added) counter += 1;
        switch (guard.direction) {
            .up => {
                if (isTopEdge(map, row, col)) {
                    return counter;
                }

                if (map[row - 1][col] == '#') {
                    guard.direction = .right;
                } else {
                    guard.position = [2]usize{ row - 1, col };
                }
            },
            .right => {
                if (isRightEdge(map, row, col)) {
                    return counter;
                }

                if (map[row][col + 1] == '#') {
                    guard.direction = .down;
                } else {
                    guard.position = [2]usize{ row, col + 1 };
                }
            },
            .down => {
                if (isBottomEdge(map, row, col)) {
                    return counter;
                }

                if (map[row + 1][col] == '#') {
                    guard.direction = .left;
                } else {
                    guard.position = [2]usize{ row + 1, col };
                }
            },
            .left => {
                if (isLeftEdge(map, row, col)) {
                    return counter;
                }

                if (map[row][col - 1] == '#') {
                    guard.direction = .up;
                } else {
                    guard.position = [2]usize{ row, col - 1 };
                }
            },
            else => @panic("unreachable"),
        }
    }
    @panic("unreachable");
}

fn doesLoop(allocator: std.mem.Allocator, map: []const []const u8, guard_: Guard) !bool {
    var guard: Guard = guard_;
    var visited = set.HashSetManaged(Guard).init(allocator);
    defer visited.deinit();
    while (true) {
        const row = guard.position[0];
        const col = guard.position[1];
        if (visited.contains(guard)) return true;
        switch (guard.direction) {
            .up => {
                if (isTopEdge(map, row, col)) {
                    return false;
                }

                if (map[row - 1][col] == '#') {
                    guard.direction = .right;
                } else {
                    _ = try visited.add(guard);
                    guard.position = [2]usize{ row - 1, col };
                }
            },
            .right => {
                if (isRightEdge(map, row, col)) {
                    return false;
                }

                if (map[row][col + 1] == '#') {
                    guard.direction = .down;
                } else {
                    _ = try visited.add(guard);
                    guard.position = [2]usize{ row, col + 1 };
                }
            },
            .down => {
                if (isBottomEdge(map, row, col)) {
                    return false;
                }

                if (map[row + 1][col] == '#') {
                    guard.direction = .left;
                } else {
                    _ = try visited.add(guard);
                    guard.position = [2]usize{ row + 1, col };
                }
            },
            .left => {
                if (isLeftEdge(map, row, col)) {
                    return false;
                }

                if (map[row][col - 1] == '#') {
                    guard.direction = .up;
                } else {
                    _ = try visited.add(guard);
                    guard.position = [2]usize{ row, col - 1 };
                }
            },
            else => @panic("unreachable"),
        }
    }
    @panic("unreachable");
}

fn partTwo(allocator: std.mem.Allocator, map: [][]u8, guard: *Guard) !u32 {
    const start = guard.*;
    var counter: u32 = 0;
    for (0..map.len) |row| {
        for (0..map[row].len) |col| {
            if (map[row][col] != '.' or (row == start.position[0] and col == start.position[0]))
                continue;
            map[row][col] = '#';
            if (try doesLoop(allocator, map, guard.*))
                counter += 1;
            map[row][col] = '.';
        }
    }
    return counter;
}

fn parseInput(allocator: std.mem.Allocator, buffer: []const u8) ![][]u8 {
    var map = std.ArrayList([]u8).init(allocator);
    var lineIter = std.mem.tokenize(u8, buffer, "\n");
    while (lineIter.next()) |line| {
        try map.append(try allocator.dupe(u8, line));
    }
    return map.toOwnedSlice();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    const map = try parseInput(allocator, input);
    defer allocator.free(map);
    defer for (map) |row| allocator.free(row);
    var guard_1 = findGuard(map);
    var guard_2 = findGuard(map);
    map[guard_1.position[0]][guard_1.position[1]] = '.';
    const part_one = try partOne(allocator, map, &guard_1);
    const part_two = try partTwo(allocator, map, &guard_2);
    std.debug.print("Part one: {}\n", .{part_one});
    std.debug.print("Part two: {}\n", .{part_two});
}

test "part-one" {
    const allocator = std.testing.allocator;
    const map = try parseInput(allocator, test_input);
    defer allocator.free(map);
    defer for (map) |row| allocator.free(row);
    var guard = findGuard(map);
    map[guard.position[0]][guard.position[1]] = '.';
    const part_one = try partOne(allocator, map, &guard);
    try std.testing.expectEqual(41, part_one);
}

test "part-two" {
    const allocator = std.testing.allocator;
    const map = try parseInput(allocator, test_input);
    defer allocator.free(map);
    defer for (map) |row| allocator.free(row);
    var guard = findGuard(map);
    map[guard.position[0]][guard.position[1]] = '.';
    const part_two = try partTwo(allocator, map, &guard);
    try std.testing.expectEqual(6, part_two);
}
