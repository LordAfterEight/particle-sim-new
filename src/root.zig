pub const element = @import("elements/element.zig");
pub const elements = struct {
    pub const sand = @import("elements/sand.zig");
    pub const dust = @import("elements/dust.zig");
};
pub const core = struct {
    pub const rendering = @import("core/rendering.zig");
    pub const renderer = @import("core/renderer.zig");
    pub const world = @import("core/world.zig");
    pub const sim = @import("core/sim.zig");
};
