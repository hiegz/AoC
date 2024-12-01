const test_input = @embedFile("test-input.txt");
const input = @embedFile("input.txt");
const std = @import("std");

fn sort_in_place(a: []u32) void {
    for (0..a.len) |i_| {
        const i = a.len - i_ - 1;
        for (0..i) |j| {
            if (a[i] < a[j]) {
                std.mem.swap(u32, &a[i], &a[j]);
            }
        }
    }
}

fn partOne(a: []u32, b: []u32) u32 {
    std.debug.assert(a.len == b.len);
    sort_in_place(a);
    sort_in_place(b);
    var sum: u32 = 0;
    for (0..a.len) |i| {
        if (a[i] > b[i]) {
            sum += (a[i] - b[i]);
        } else {
            sum += (b[i] - a[i]);
        }
    }
    return sum;
}

fn partTwo(a: []u32, b: []u32) !u32 {
    const allocator = std.heap.c_allocator;
    var hash_map = std.AutoHashMap(u32, u32).init(allocator);
    defer hash_map.deinit();
    for (b) |item| {
        if (hash_map.getPtr(item)) |value| {
            value.* += 1;
        } else {
            try hash_map.put(item, 1);
        }
    }
    var sum: u32 = 0;
    for (a) |item| {
        const k: u32 = hash_map.get(item) orelse 0;
        sum += item * k;
    }
    return sum;
}

pub fn parseInput(allocator: std.mem.Allocator, buffer: []const u8) !struct { a: std.ArrayList(u32), b: std.ArrayList(u32) } {
    var a = std.ArrayList(u32).init(allocator);
    errdefer a.deinit();
    var b = std.ArrayList(u32).init(allocator);
    errdefer b.deinit();

    var lineIter = std.mem.tokenize(u8, buffer, "\n");
    while (lineIter.next()) |line| {
        var numberIter = std.mem.tokenize(u8, line, " ");
        try a.append(try std.fmt.parseInt(u32, numberIter.next().?, 10));
        try b.append(try std.fmt.parseInt(u32, numberIter.next().?, 10));
    }

    return .{ .a = a, .b = b };
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const lists = try parseInput(allocator, input);
    defer lists.a.deinit();
    defer lists.b.deinit();

    const part_one = partOne(lists.a.items, lists.b.items);
    const part_two = partTwo(lists.a.items, lists.b.items);

    std.debug.print("Part one: {any}\n", .{part_one});
    std.debug.print("Part two: {any}\n", .{part_two});
}

test "part-one" {
    const allocator = std.testing.allocator;
    const lists = try parseInput(allocator, test_input);
    defer lists.a.deinit();
    defer lists.b.deinit();

    const result = partOne(lists.a.items, lists.b.items);
    try std.testing.expectEqual(11, result);
}

test "part-two" {
    const allocator = std.testing.allocator;
    const lists = try parseInput(allocator, test_input);
    defer lists.a.deinit();
    defer lists.b.deinit();

    const result = try partTwo(lists.a.items, lists.b.items);
    try std.testing.expectEqual(31, result);
}
