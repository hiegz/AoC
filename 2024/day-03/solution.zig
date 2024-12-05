const test_input = @embedFile("test-input.txt");
const input = @embedFile("input.txt");
const std = @import("std");

const c = @cImport({
    @cInclude("regex.h");
    @cInclude("regex_mem.h");
});

const Command = union(enum) {
    mul: [2]u32,
    do,
    dont,
};

fn partOne(commands: []const Command) u32 {
    var sum: u32 = 0;
    for (commands) |command| {
        switch (command) {
            .mul => sum += (command.mul[0] * command.mul[1]),
            else => {},
        }
    }
    return sum;
}

fn partTwo(commands: []const Command) u32 {
    var sum: u32 = 0;
    var is_enabled: bool = true;
    for (commands) |command| {
        switch (command) {
            .mul => sum += if (is_enabled) (command.mul[0] * command.mul[1]) else 0,
            .do => is_enabled = true,
            .dont => is_enabled = false,
        }
    }
    return sum;
}

fn parseInput(allocator: std.mem.Allocator, buffer: []const u8) ![]Command {
    var commands = std.ArrayList(Command).init(allocator);

    var rt: c_int = 0;
    const regex_mul_string = "(mul)\\(([0-9]{1,3}),([0-9]{1,3})\\)";
    const regex_do_string = "(do)\\(\\)";
    const regex_dont_string = "(don't)\\(\\)";
    const regex_string = regex_mul_string ++ "|" ++ regex_do_string ++ "|" ++ regex_dont_string;
    const regex_groups: usize = 6; // [match, command, arg1, arg2]
    var regex_group_arr: [regex_groups]c.regmatch_t = undefined;
    const regex: *c.regex_t = c.alloc_regex_t().?;
    defer c.free_regex_t(regex);

    rt = c.regcomp(regex, regex_string, c.REG_EXTENDED);
    if (0 != rt) {
        const error_buffer: [*c]u8 = @constCast((&std.mem.zeroes([128]u8)).ptr);
        _ = c.regerror(rt, regex, error_buffer, 128);
        std.log.err("{s}\n", .{error_buffer});
        return error.RegexError;
    }

    var regex_string_ptr = buffer.ptr;

    while (true) {
        rt = c.regexec(regex, regex_string_ptr, regex_groups, @ptrCast((&regex_group_arr).ptr), 0);
        if (c.REG_NOMATCH == rt) {
            break;
        }
        if (0 != rt) {
            var error_buffer: [128]u8 = undefined;
            _ = c.regerror(rt, regex, @ptrCast((&error_buffer).ptr), 128);
            std.log.err("{s}\n", .{error_buffer});
            return error.RegexError;
        }

        var command: Command = undefined;

        // check if we got the mul(arg1, arg2) group
        if (regex_group_arr[1].rm_so != -1 and regex_group_arr[1].rm_eo != -1) {
            command = .{ .mul = std.mem.zeroes([2]u32) };
            const arg1_start_offset = @as(usize, @intCast(regex_group_arr[2].rm_so));
            const arg1_end_offset = @as(usize, @intCast(regex_group_arr[2].rm_eo));
            const arg1_slice = regex_string_ptr[arg1_start_offset..arg1_end_offset];
            command.mul[0] = try std.fmt.parseInt(u32, arg1_slice, 10);
            const arg2_start_offset = @as(usize, @intCast(regex_group_arr[3].rm_so));
            const arg2_end_offset = @as(usize, @intCast(regex_group_arr[3].rm_eo));
            const arg2_slice = regex_string_ptr[arg2_start_offset..arg2_end_offset];
            command.mul[1] = try std.fmt.parseInt(u32, arg2_slice, 10);
        }
        // check if we got the do() group
        else if (regex_group_arr[4].rm_so != -1 and regex_group_arr[4].rm_eo != -1) {
            command = .do;
        }
        // check if we got the don't() group
        else if (regex_group_arr[5].rm_so != -1 and regex_group_arr[5].rm_eo != -1) {
            command = .dont;
        }

        regex_string_ptr += @as(usize, @intCast(regex_group_arr[0].rm_eo));
        try commands.append(command);
    }

    return commands.toOwnedSlice();
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    const commands = try parseInput(allocator, input);
    defer allocator.free(commands);
    const part_one = partOne(commands);
    const part_two = partTwo(commands);
    std.debug.print("Part one: {}\n", .{part_one});
    std.debug.print("Part two: {}\n", .{part_two});
}

test "part-one" {
    const allocator = std.testing.allocator;
    const commands = try parseInput(allocator, test_input);
    defer allocator.free(commands);
    const part_one = partOne(commands);
    try std.testing.expectEqual(161, part_one);
}

test "part-two" {
    const allocator = std.testing.allocator;
    const commands = try parseInput(allocator, test_input);
    defer allocator.free(commands);
    const part_two = partTwo(commands);
    try std.testing.expectEqual(48, part_two);
}
