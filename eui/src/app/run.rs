use crate::app::options::AppOptions;
use crate::core::context::Context;
use crate::quick::ui::UI;

pub fn run<F>(build_ui: F)
where
    F: FnMut(&mut Context, &mut UI<'_>) + 'static,
{
    run_with_options(build_ui, AppOptions::default());
}

pub fn run_with_options<F>(build_ui: F, options: AppOptions)
where
    F: FnMut(&mut Context, &mut UI<'_>) + 'static,
{
    crate::app::event_loop::run_app(build_ui, options);
}
