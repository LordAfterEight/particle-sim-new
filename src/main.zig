const rl = @import("raylib");
const particle_sim = @import("particle_sim_new");

const std = @import("std");

pub fn main() anyerror!void {

    rl.initWindow(particle_sim.core.rendering.SCREEN_WIDTH, particle_sim.core.rendering.SCREEN_HEIGHT, "Ziggy Pixels");
    defer rl.closeWindow();

    rl.setTargetFPS(60);

    particle_sim.core.renderer.init();
    defer particle_sim.core.renderer.deinit();

    var viewport = particle_sim.core.world.ViewPort.init(std.heap.page_allocator);
    defer viewport.deinit();

    const sand = particle_sim.core.sim.Particle.new(null, particle_sim.elements.sand.Sand);
    const dust = particle_sim.core.sim.Particle.new(null, particle_sim.elements.dust.Dust);


    while (!rl.windowShouldClose()) {
        rl.beginDrawing();
        defer rl.endDrawing();
        rl.clearBackground(.black);

        if (rl.isMouseButtonDown(.left)) {
            viewport.spawn(.{ .x = @floatFromInt(rl.getMouseX()), .y = @floatFromInt(rl.getMouseY()), .z = 0.0 }, sand);
        }
        if (rl.isMouseButtonDown(.right)) {
            viewport.spawn(.{ .x = @floatFromInt(rl.getMouseX()), .y = @floatFromInt(rl.getMouseY()), .z = 0.0 }, dust);
        }

        try viewport.render();
    }
}
