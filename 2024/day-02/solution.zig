const test_input = @embedFile("test-input.txt");
const input = @embedFile("input.txt");
const std = @import("std");

fn validateLevels(a: u32, b: u32, is_increasing: bool) bool {
    return (a != b) and
        ((a < b and is_increasing and b - a <= 3) or
        (a > b and !is_increasing and a - b <= 3));
}

fn validateReport(report: []const u32) bool {
    std.debug.assert(report.len > 1);
    const is_increasing: bool = (report[0] < report[1]);
    for (1..report.len) |idx| {
        if (!validateLevels(report[idx - 1], report[idx], is_increasing)) {
            return false;
        }
    }
    return true;
}

fn validateBadReport(report: []const u32, report_index: usize) bool {
    var prev_idx: usize = if (report_index != 0) 0 else 1;
    std.debug.assert(report.len > prev_idx + 1);
    const is_increasing: bool = if (report_index == prev_idx + 1)
        (report[prev_idx] < report[prev_idx + 2])
    else
        (report[prev_idx] < report[prev_idx + 1]);
    for (prev_idx + 1..report.len) |idx| {
        if (idx == report_index) continue;
        if (!validateLevels(report[prev_idx], report[idx], is_increasing)) {
            return false;
        }
        prev_idx = idx;
    }
    return true;
}

fn partOne(reports: []const []const u32) usize {
    var safe_counter: usize = 0;
    for (reports) |report| {
        if (validateReport(report)) {
            safe_counter += 1;
        }
    }
    return safe_counter;
}

fn partTwo(allocator: std.mem.Allocator, reports: []const []const u32) !usize {
    var safe_counter: usize = 0;
    for (reports) |report| {
        if (validateReport(report)) {
            safe_counter += 1;
            continue;
        }

        const bad_report = try allocator.alloc(u32, report.len - 1);
        defer allocator.free(bad_report);
        for (0..report.len) |idx| {
            for (0..report.len) |jdx| {
                if (idx == jdx) continue;
                if (idx > jdx) {
                    bad_report[jdx] = report[jdx];
                } else {
                    bad_report[jdx - 1] = report[jdx];
                }
            }

            if (validateReport(bad_report)) {
                safe_counter += 1;
                break;
            }
        }
    }
    return safe_counter;
}

fn partTwoInPlace(reports: []const []const u32) !usize {
    var safe_counter: usize = 0;
    for (reports) |report| {
        if (validateReport(report)) {
            safe_counter += 1;
            continue;
        }

        for (0..report.len) |idx| {
            if (validateBadReport(report, idx)) {
                safe_counter += 1;
                break;
            }
        }
    }
    return safe_counter;
}

fn parseInput(allocator: std.mem.Allocator, buffer: []const u8) ![][]u32 {
    var array = std.ArrayList([]u32).init(allocator);
    errdefer array.deinit();
    errdefer for (array.items) |report| {
        allocator.free(report);
    };
    var lineIter = std.mem.tokenize(u8, buffer, "\n");
    while (lineIter.next()) |line| {
        var report = std.ArrayList(u32).init(allocator);
        errdefer report.deinit();
        var tokenIter = std.mem.tokenize(u8, line, " ");
        while (tokenIter.next()) |token| {
            try report.append(try std.fmt.parseInt(u32, token, 10));
        }
        try array.append(try report.toOwnedSlice());
    }
    return try array.toOwnedSlice();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const reports = try parseInput(allocator, input);
    defer allocator.free(reports);
    defer for (reports) |report| {
        allocator.free(report);
    };

    const part_one = partOne(reports);
    const part_two = try partTwo(allocator, reports);
    const part_two_in_place = try partTwoInPlace(reports);

    std.debug.print("Part one: {any}\n", .{part_one});
    std.debug.print("Part two: {any}\n", .{part_two});
    std.debug.print("Part two (in-place): {any}\n", .{part_two_in_place});
}

test "part-one" {
    const allocator = std.testing.allocator;

    const reports = try parseInput(allocator, test_input);
    defer allocator.free(reports);
    defer for (reports) |report| {
        allocator.free(report);
    };

    const result = partOne(reports);
    try std.testing.expectEqual(2, result);
}

test "part-two" {
    const allocator = std.testing.allocator;

    const reports = try parseInput(allocator, test_input);
    defer allocator.free(reports);
    defer for (reports) |report| {
        allocator.free(report);
    };

    const result = partTwoInPlace(reports);
    try std.testing.expectEqual(4, result);
}
