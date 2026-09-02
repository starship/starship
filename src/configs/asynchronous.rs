use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::Duration;

/// Streaming prompt timing settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct AsynchronousConfig {
    /// Enables refinements after the first paint.
    pub enabled: bool,
    /// Maximum refinement batching delay in milliseconds.
    pub bus: u64,
    /// Batches refinements using session timing estimates.
    pub adaptive: bool,
    /// Refresh periods for dynamic modules.
    pub dynamic: DynamicPeriodsConfig,
}

impl Default for AsynchronousConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bus: 100,
            adaptive: true,
            dynamic: DynamicPeriodsConfig::default(),
        }
    }
}

macro_rules! dynamic_periods {
    ($(
        $(#[$attribute:meta])*
        $module:ident;
    )+) => {
        /// Dynamic module refresh periods.
        #[derive(Clone, Debug, PartialEq, Serialize)]
        #[cfg_attr(
            feature = "config-schema",
            derive(schemars::JsonSchema),
            schemars(deny_unknown_fields)
        )]
        #[serde(default)]
        pub struct DynamicPeriodsConfig {
            $(
                $(#[$attribute])*
                #[cfg_attr(
                    feature = "config-schema",
                    schemars(with = "u64", range(min = 1, max = 86_400_000))
                )]
                pub $module: RefreshPeriod,
            )+
        }

        impl Default for DynamicPeriodsConfig {
            fn default() -> Self {
                Self {
                    $($(#[$attribute])* $module: default_period(stringify!($module)),)+
                }
            }
        }

        impl DynamicPeriodsConfig {
            /// Returns a module's refresh period.
            #[must_use]
            pub fn period_for(&self, module: &str) -> Option<Duration> {
                match module {
                    $($(#[$attribute])* stringify!($module) => Some(self.$module.get()),)+
                    _ => None,
                }
            }
        }

        impl<'de> Deserialize<'de> for DynamicPeriodsConfig {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                #[derive(Default, Deserialize)]
                #[serde(default)]
                struct Unvalidated {
                    $($(#[$attribute])* $module: Option<toml::Value>,)+
                }

                let configured = Unvalidated::deserialize(deserializer)?;
                let defaults = DynamicPeriodsConfig::default();

                Ok(Self {
                    $($(#[$attribute])* $module: {
                        let value = configured.$module.as_ref();
                        value
                            .and_then(toml::Value::as_integer)
                            .and_then(|milliseconds| u64::try_from(milliseconds).ok())
                            .and_then(|milliseconds| RefreshPeriod::try_from(milliseconds).ok())
                            .unwrap_or_else(|| {
                                if value.is_some() {
                            log::warn!(
                                "[async.dynamic] {}: invalid interval — keeping the default of {}ms",
                                stringify!($module),
                                defaults.$module.get().as_millis(),
                            );
                                }
                                defaults.$module
                            })
                    }),+
                })
            }
        }
    };
}

dynamic_periods! {
    time;
    #[cfg(feature = "battery")]
    battery;
    memory_usage;
    localip;
}

fn default_period(module: &str) -> RefreshPeriod {
    let Some(crate::modules::Cadence::Dynamic { period }) = crate::modules::cadence(module) else {
        panic!("{module} is configured as dynamic without a dynamic cadence")
    };
    RefreshPeriod(period)
}

const MAX_REFRESH_PERIOD_MILLISECONDS: u64 = 86_400_000;

/// A dynamic-module refresh period of at most one day.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RefreshPeriod(Duration);

impl RefreshPeriod {
    /// Returns the interval.
    #[must_use]
    pub const fn get(self) -> Duration {
        self.0
    }
}

impl TryFrom<u64> for RefreshPeriod {
    type Error = &'static str;

    fn try_from(milliseconds: u64) -> Result<Self, Self::Error> {
        if milliseconds == 0 {
            return Err("a refresh period must be nonzero");
        }
        if milliseconds > MAX_REFRESH_PERIOD_MILLISECONDS {
            return Err("a refresh period must be at most one day");
        }
        Ok(Self(Duration::from_millis(milliseconds)))
    }
}

impl Serialize for RefreshPeriod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let milliseconds = u64::try_from(self.0.as_millis())
            .expect("a refresh period always originates as u64 milliseconds");
        milliseconds.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::{Cadence, cadence};

    #[test]
    fn every_dynamic_module_can_be_given_a_period() {
        for module in crate::module::ALL_MODULES {
            if matches!(cadence(module), Some(Cadence::Dynamic { .. })) {
                assert!(
                    DynamicPeriodsConfig::default().period_for(module).is_some(),
                    "{module} is dynamic but has no configurable period"
                );
            }
        }
    }

    #[test]
    fn a_module_that_is_not_dynamic_has_no_period() {
        for module in ["git_status", "character", "custom.anything", "nonsense"] {
            assert_eq!(None, DynamicPeriodsConfig::default().period_for(module));
        }
    }

    #[test]
    fn streaming_is_on_by_default_and_paces_itself_over_a_tenth_of_a_second() {
        let configuration = AsynchronousConfig::default();

        assert!(configuration.enabled);
        assert!(configuration.adaptive);
        assert_eq!(100, configuration.bus);
    }

    #[test]
    fn an_unusable_period_falls_back_to_the_default_for_that_module_alone() {
        let periods = toml::from_str::<DynamicPeriodsConfig>("time = 0\nlocalip = 7000")
            .expect("an unusable period is warned about, not propagated");

        assert_eq!(DynamicPeriodsConfig::default().time, periods.time);
        assert_eq!(
            Some(Duration::from_millis(7_000)),
            periods.period_for("localip")
        );
    }

    #[test]
    fn an_unusable_period_does_not_discard_the_rest_of_the_root_table() {
        use crate::config::ModuleConfig;

        let table = toml::toml! {
            add_newline = false
            format = "$directory$character"
            [async.dynamic]
            time = 0
        };
        let root = crate::configs::StarshipRootConfig::load(&table);

        assert_eq!(
            "$directory$character", root.format,
            "an unusable period must not discard the configured format"
        );
        assert!(
            !root.add_newline,
            "an unusable period must not discard the other root settings"
        );
        assert_eq!(
            DynamicPeriodsConfig::default().time,
            root.asynchronous.dynamic.time,
            "the unusable period itself falls back to its default"
        );
    }

    #[test]
    fn refresh_period_requires_a_value_within_one_day() {
        assert!(RefreshPeriod::try_from(0).is_err());
        assert!(RefreshPeriod::try_from(MAX_REFRESH_PERIOD_MILLISECONDS + 1).is_err());
    }

    #[test]
    fn refresh_period_accepts_one_day() {
        assert_eq!(
            Ok(RefreshPeriod(Duration::from_millis(
                MAX_REFRESH_PERIOD_MILLISECONDS
            ))),
            RefreshPeriod::try_from(MAX_REFRESH_PERIOD_MILLISECONDS)
        );
    }

    #[test]
    fn refresh_period_round_trips_through_serialization() {
        let original = DynamicPeriodsConfig {
            time: RefreshPeriod::try_from(12_345).expect("in range"),
            ..DynamicPeriodsConfig::default()
        };

        let serialized = toml::to_string(&original).expect("a config table serializes");
        let deserialized: DynamicPeriodsConfig =
            toml::from_str(&serialized).expect("what was just written must parse back");
        assert_eq!(original, deserialized);
    }
}
