use rand::seq::SliceRandom;

use crate::{
    config::ModuleConfig, configs::random::RandomConfig, context::Context,
    formatter::StringFormatter, module::Module,
};

pub fn module<'ctx>(name: &str, ctx: &'ctx Context) -> Option<Module<'ctx>> {
    let cfg = ctx.config.get_random_module_config(name).or_else(|| {
        log::debug!("tried to load top level random config '{name}' but config doesn't exist");
        None
    })?;
    let mut module = Module::new(&format!("random.{name}"), "<random module>", Some(cfg));
    let cfg = RandomConfig::load(cfg);

    let mut rng = rand::thread_rng();
    let symbol = cfg.symbols.choose(&mut rng);
    let style = cfg.styles.choose(&mut rng);
    let parsed = StringFormatter::new(cfg.format).and_then(|f| {
        f.map_meta(|var, _| match var {
            "symbol" => symbol.copied(),
            _ => None,
        })
        .map_style(|var| match var {
            "style" => style.copied().map(Ok),
            _ => None,
        })
        .parse(None, Some(ctx))
    });
    match parsed {
        Ok(segs) => module.set_segments(segs),
        Err(e) => {
            log::warn!("Error in module `random.{name}`:\n{e}");
        }
    }
    Some(module)
}
