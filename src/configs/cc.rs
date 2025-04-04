use serde::{Deserialize, Serialize};

#[cfg(feature = "config-schema")]
use schemars::{
    JsonSchema, SchemaGenerator, schema::InstanceType, schema::Schema, schema::SchemaObject,
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CcConfig<'a, T> {
    #[serde(skip)]
    pub marker: std::marker::PhantomData<T>,

    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub commands: Vec<Vec<&'a str>>,
}

#[cfg(feature = "config-schema")]
impl<'a> JsonSchema for CcConfig<'a, crate::configs::c::CConfig<'a>> {
    fn schema_name() -> String {
        "CConfig".to_owned()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(concat!(
            module_path!(),
            "::CcConfig<'a, crate::configs::c::CConfig<'a>>"
        ))
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let container_default = Self::default();
        generate_ccconfig_schema(container_default, generator)
    }
}

#[cfg(feature = "config-schema")]
impl<'a> JsonSchema for CcConfig<'a, crate::configs::cpp::CppConfig<'a>> {
    fn schema_name() -> String {
        "CppConfig".to_owned()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(concat!(
            module_path!(),
            "::CcConfig<'a, crate::configs::cpp::CppConfig<'a>>"
        ))
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let container_default = Self::default();
        generate_ccconfig_schema(container_default, generator)
    }
}

#[cfg(feature = "config-schema")]
fn generate_ccconfig_schema<'a, T>(
    container_default: CcConfig<T>,
    generator: &mut SchemaGenerator,
) -> Schema {
    let mut schema_object = SchemaObject {
        instance_type: Some(InstanceType::Object.into()),
        ..Default::default()
    };
    let object_validation = schema_object.object();
    object_validation.additional_properties = Some(Box::new(false.into()));

    insert_schema_property(
        generator,
        object_validation,
        "format",
        &container_default.format,
    );
    insert_schema_property(
        generator,
        object_validation,
        "version_format",
        &container_default.version_format,
    );
    insert_schema_property(
        generator,
        object_validation,
        "style",
        &container_default.style,
    );
    insert_schema_property(
        generator,
        object_validation,
        "symbol",
        &container_default.symbol,
    );
    insert_schema_property(
        generator,
        object_validation,
        "disabled",
        &container_default.disabled,
    );
    insert_schema_property(
        generator,
        object_validation,
        "detect_extensions",
        &container_default.detect_extensions,
    );
    insert_schema_property(
        generator,
        object_validation,
        "detect_files",
        &container_default.detect_files,
    );
    insert_schema_property(
        generator,
        object_validation,
        "detect_folders",
        &container_default.detect_folders,
    );
    insert_schema_property(
        generator,
        object_validation,
        "commands",
        &container_default.commands,
    );

    Schema::Object(schema_object)
}

#[cfg(feature = "config-schema")]
fn insert_schema_property<T: JsonSchema + serde::Serialize>(
    generator: &mut SchemaGenerator,
    object_validation: &mut schemars::schema::ObjectValidation,
    name: &str,
    default_value: &T,
) {
    use schemars::_private::{
        MaybeSerializeWrapper, insert_object_property, metadata::add_default,
    };

    insert_object_property::<&'_ str>(
        object_validation,
        name,
        true,
        false,
        add_default(
            generator.subschema_for::<T>(),
            MaybeSerializeWrapper(default_value).maybe_to_value(),
        ),
    );
}
