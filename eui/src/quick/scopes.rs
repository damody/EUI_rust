use crate::core::context::Context;
use crate::rect::Rect;

pub struct RegionScope<'a> {
    ctx: &'a mut Context,
    shell_rect: Rect,
    active: bool,
}

impl<'a> RegionScope<'a> {
    pub fn new(ctx: &'a mut Context, rect: Rect) -> Self {
        ctx.push_layout_rect(rect);
        Self { ctx, shell_rect: rect, active: true }
    }

    pub fn shell(&self) -> Rect {
        self.shell_rect
    }

    pub fn content(&self) -> Rect {
        self.ctx.layout_rect()
    }

    pub fn dock_left(&mut self, width: f32) -> Rect {
        self.ctx.dock_left(width)
    }

    pub fn dock_right(&mut self, width: f32) -> Rect {
        self.ctx.dock_right(width)
    }

    pub fn dock_top(&mut self, height: f32) -> Rect {
        self.ctx.dock_top(height)
    }

    pub fn dock_bottom(&mut self, height: f32) -> Rect {
        self.ctx.dock_bottom(height)
    }

    pub fn ctx(&mut self) -> &mut Context {
        self.ctx
    }

    pub fn end(mut self) {
        self.close();
    }

    fn close(&mut self) {
        if self.active {
            self.ctx.pop_layout_rect();
            self.active = false;
        }
    }
}

impl<'a> Drop for RegionScope<'a> {
    fn drop(&mut self) {
        self.close();
    }
}

pub struct ClipScope<'a> {
    ctx: &'a mut Context,
    active: bool,
}

impl<'a> ClipScope<'a> {
    pub fn new(ctx: &'a mut Context, rect: Rect) -> Self {
        ctx.push_clip(rect);
        Self { ctx, active: true }
    }

    pub fn ctx(&mut self) -> &mut Context {
        self.ctx
    }

    pub fn end(mut self) {
        self.close();
    }

    fn close(&mut self) {
        if self.active {
            self.ctx.pop_clip();
            self.active = false;
        }
    }
}

impl<'a> Drop for ClipScope<'a> {
    fn drop(&mut self) {
        self.close();
    }
}

pub struct FlexRowScope<'a> {
    ctx: &'a mut Context,
    active: bool,
}

impl<'a> FlexRowScope<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx, active: true }
    }

    pub fn ctx(&mut self) -> &mut Context {
        self.ctx
    }

    pub fn end(mut self) {
        self.close();
    }

    fn close(&mut self) {
        if self.active {
            self.ctx.end_flex_row();
            self.active = false;
        }
    }
}

impl<'a> Drop for FlexRowScope<'a> {
    fn drop(&mut self) {
        self.close();
    }
}
