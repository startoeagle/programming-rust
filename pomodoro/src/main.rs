use std::time::Duration;

use druid::{
    widget::{Align, Flex, Label},
    AppLauncher, Data, Env, Lens, RenderContext, Widget, WidgetExt, WidgetPod, WindowDesc,
};
use druid::{Event, TimerToken};

struct Timer {
    timer_id: TimerToken,
    time: WidgetPod<u32, Align<u32>>,
}

static INTERVAL: Duration = Duration::from_millis(10);

impl Widget<u32> for Timer {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut u32,
        _env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                self.timer_id = ctx.request_timer(INTERVAL);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    *data += 1;
                    self.timer_id = ctx.request_timer(INTERVAL);
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &u32,
        _env: &Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &u32, _data: &u32, _env: &Env) {}

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &u32,
        _env: &Env,
    ) -> druid::Size {
        bc.constrain((500.0, 500.0))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &u32, env: &Env) {
        // let mut label = Label::new(format!("{}", data));
        // label.paint(ctx, data, env);
        self.time.paint(ctx, data, env);
    }
}

fn main() {
    let main_window = WindowDesc::new(|| {
        let label = Label::new(|data: &u32, _env: &Env| format!("{}", data));
        let flex = Flex::column()
            .with_child(label)
            .align_vertical(druid::UnitPoint::CENTER)
            .center();

        Timer {
            time: WidgetPod::new(flex),
            timer_id: TimerToken::INVALID,
        }
    })
    .title("My first app")
    .with_min_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(0u32)
        .expect("failed to launch application");
}
