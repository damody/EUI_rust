use crate::core::context::Context;
use crate::core::foundation::FlexLength;
use crate::rect::Rect;

pub struct LinearLayout {
    pub rect: Rect,
    pub horizontal: bool,
    pub gap: f32,
    pub items: Vec<FlexLength>,
}

impl LinearLayout {
    pub fn row(rect: Rect) -> Self {
        Self { rect, horizontal: true, gap: 8.0, items: Vec::new() }
    }

    pub fn column(rect: Rect) -> Self {
        Self { rect, horizontal: false, gap: 8.0, items: Vec::new() }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn item(mut self, item: FlexLength) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: &[FlexLength]) -> Self {
        self.items.extend_from_slice(items);
        self
    }

    pub fn resolve(&self) -> Vec<Rect> {
        let n = self.items.len();
        if n == 0 { return Vec::new(); }

        let total_gap = if n > 1 { self.gap * (n as f32 - 1.0) } else { 0.0 };
        let avail = if self.horizontal {
            (self.rect.w - total_gap).max(0.0)
        } else {
            (self.rect.h - total_gap).max(0.0)
        };

        let mut sizes = vec![0.0f32; n];
        let mut total_fixed = 0.0f32;
        let mut total_flex = 0.0f32;

        for (i, item) in self.items.iter().enumerate() {
            match item {
                FlexLength::Fixed(px) => { sizes[i] = *px; total_fixed += px; }
                FlexLength::Flex(w) => { total_flex += w; }
                FlexLength::Content { min, .. } => { sizes[i] = *min; total_fixed += min; }
            }
        }

        let remaining = (avail - total_fixed).max(0.0);
        if total_flex > 0.0 {
            for (i, item) in self.items.iter().enumerate() {
                if let FlexLength::Flex(w) = item {
                    sizes[i] = remaining * w / total_flex;
                }
            }
        }

        let mut rects = Vec::with_capacity(n);
        let mut offset = 0.0f32;
        for &size in &sizes {
            let rect = if self.horizontal {
                Rect::new(self.rect.x + offset, self.rect.y, size, self.rect.h)
            } else {
                Rect::new(self.rect.x, self.rect.y + offset, self.rect.w, size)
            };
            rects.push(rect);
            offset += size + self.gap;
        }
        rects
    }
}

pub struct GridLayout {
    pub rect: Rect,
    pub columns: usize,
    pub rows: usize,
    pub gap: f32,
}

impl GridLayout {
    pub fn new(rect: Rect, columns: usize) -> Self {
        Self { rect, columns: columns.max(1), rows: 0, gap: 8.0 }
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn cell(&self, index: usize) -> Rect {
        let col = index % self.columns;
        let row = index / self.columns;
        let col_gap = self.gap * (self.columns as f32 - 1.0);
        let cell_w = ((self.rect.w - col_gap) / self.columns as f32).max(0.0);
        let cell_h = if self.rows > 0 {
            let row_gap = self.gap * (self.rows as f32 - 1.0);
            ((self.rect.h - row_gap) / self.rows as f32).max(0.0)
        } else {
            cell_w
        };
        let x = self.rect.x + col as f32 * (cell_w + self.gap);
        let y = self.rect.y + row as f32 * (cell_h + self.gap);
        Rect::new(x, y, cell_w, cell_h)
    }
}

pub struct StackLayout {
    pub rect: Rect,
}

impl StackLayout {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }

    pub fn layer(&self) -> Rect {
        self.rect
    }
}

pub struct ViewBuilder<'a> {
    ctx: &'a mut Context,
    rect: Rect,
}

impl<'a> ViewBuilder<'a> {
    pub fn new(ctx: &'a mut Context, rect: Rect) -> Self {
        Self { ctx, rect }
    }

    pub fn begin<F: FnOnce(&mut Context)>(self, f: F) {
        self.ctx.push_layout_rect(self.rect);
        f(self.ctx);
        self.ctx.pop_layout_rect();
    }
}
