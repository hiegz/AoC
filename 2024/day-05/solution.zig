const test_input = @embedFile("test-input.txt");
const input = @embedFile("input.txt");
const std = @import("std");
const set = @import("ziglangSet");

fn partOne(allocator: std.mem.Allocator, ordering_rules: []const [2]u32, updates: []const []const u32) !u32 {
    var visited = set.HashSetManaged(u32).init(allocator);
    defer visited.deinit();
    var primap = std.AutoHashMap(u32, std.ArrayList(u32)).init(allocator);
    defer primap.deinit();
    defer {
        var iter = primap.valueIterator();
        while (iter.next()) |value| value.deinit();
    }

    for (ordering_rules) |rule| {
        if (primap.getPtr(rule[0])) |list| {
            try list.*.append(rule[1]);
        } else {
            var list = std.ArrayList(u32).init(allocator);
            try list.append(rule[1]);
            try primap.put(rule[0], list);
        }
    }

    var sum: u32 = 0;

    parent: for (updates) |update| {
        visited.clearRetainingCapacity();
        std.debug.assert(update.len % 2 != 0);
        for (update) |page| {
            _ = try visited.add(page);
            if (primap.get(page)) |secondaries| {
                for (secondaries.items) |secondary| {
                    if (visited.contains(secondary)) {
                        continue :parent;
                    }
                }
            }
        }
        sum += update[update.len / 2];
    }

    return sum;
}

fn partTwo(allocator: std.mem.Allocator, ordering_rules: []const [2]u32, updates_: []const []const u32) !u32 {
    var updates = try allocator.alloc([]u32, updates_.len);
    defer allocator.free(updates);
    for (updates_, 0..) |update, i| {
        updates[i] = try allocator.dupe(u32, update);
    }
    defer for (updates) |update| allocator.free(update);

    var visited = set.HashSetManaged(u32).init(allocator);
    defer visited.deinit();
    var primap = std.AutoHashMap(u32, std.ArrayList(u32)).init(allocator);
    defer primap.deinit();
    defer {
        var iter = primap.valueIterator();
        while (iter.next()) |value| value.deinit();
    }

    for (ordering_rules) |rule| {
        if (primap.getPtr(rule[0])) |list| {
            try list.*.append(rule[1]);
        } else {
            var list = std.ArrayList(u32).init(allocator);
            try list.append(rule[1]);
            try primap.put(rule[0], list);
        }
    }

    var sum: u32 = 0;

    var i: usize = 0;
    parent: while (i < updates.len) {
        visited.clearRetainingCapacity();
        std.debug.assert(updates[i].len % 2 != 0);
        for (0..updates[i].len) |j| {
            _ = try visited.add(updates[i][j]);
            if (primap.get(updates[i][j])) |secondaries| {
                for (secondaries.items) |secondary| {
                    if (visited.contains(secondary)) {
                        for (0..updates[i].len) |k| {
                            if (updates[i][k] == secondary) {
                                const temp = secondary;
                                updates[i][k] = updates[i][j];
                                updates[i][j] = temp;
                                break;
                            }
                        }
                        continue :parent;
                    }
                }
            }
        }
        sum += updates[i][updates[i].len / 2];
        i += 1;
    }

    return sum - try partOne(allocator, ordering_rules, updates_);
}

fn parseInput(allocator: std.mem.Allocator, buffer: []const u8) !struct { ordering_rules: [][2]u32, updates: [][]u32 } {
    var ordering_rules = std.ArrayList([2]u32).init(allocator);
    var updates = std.ArrayList([]u32).init(allocator);
    var blockIter = std.mem.tokenizeSequence(u8, buffer, "\n\n");
    var lineIter = std.mem.tokenize(u8, blockIter.next().?, "\n");
    while (lineIter.next()) |line| {
        var tokenIter = std.mem.tokenize(u8, line, "|");
        const left = try std.fmt.parseInt(u32, tokenIter.next().?, 10);
        const right = try std.fmt.parseInt(u32, tokenIter.next().?, 10);
        try ordering_rules.append([2]u32{ left, right });
    }
    lineIter = std.mem.tokenize(u8, blockIter.next().?, "\n");
    while (lineIter.next()) |line| {
        var update = std.ArrayList(u32).init(allocator);
        var tokenIter = std.mem.tokenize(u8, line, ",");
        while (tokenIter.next()) |token| {
            try update.append(try std.fmt.parseInt(u32, token, 10));
        }
        try updates.append(try update.toOwnedSlice());
    }
    return .{
        .ordering_rules = try ordering_rules.toOwnedSlice(),
        .updates = try updates.toOwnedSlice(),
    };
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    const in = try parseInput(allocator, input);
    defer allocator.free(in.ordering_rules);
    defer allocator.free(in.updates);
    defer for (in.updates) |update| allocator.free(update);
    const part_one = partOne(allocator, in.ordering_rules, in.updates);
    const part_two = partTwo(allocator, in.ordering_rules, in.updates);
    std.debug.print("Part one: {any}\n", .{part_one});
    std.debug.print("Part two: {any}\n", .{part_two});
}

test "part-one" {
    const allocator = std.testing.allocator;
    const in = try parseInput(allocator, test_input);
    defer allocator.free(in.ordering_rules);
    defer allocator.free(in.updates);
    defer for (in.updates) |update| allocator.free(update);
    const part_one = partOne(allocator, in.ordering_rules, in.updates);
    try std.testing.expectEqual(143, part_one);
}

test "part-two" {
    const allocator = std.testing.allocator;
    const in = try parseInput(allocator, test_input);
    defer allocator.free(in.ordering_rules);
    defer allocator.free(in.updates);
    defer for (in.updates) |update| allocator.free(update);
    const part_two = partTwo(allocator, in.ordering_rules, in.updates);
    try std.testing.expectEqual(123, part_two);
}
