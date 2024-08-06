use config::{language_option, low_accuracy_mode, LanguageOption};
use detector::init_detector;
#[cfg(not(feature = "integration-tests"))]
use redis_module::{configuration::ConfigurationFlags, redis_module, Context, RedisString, Status};

#[cfg(not(feature = "integration-tests"))]
mod command;
mod config;
mod detector;

/// Initialize the module.
#[cfg(not(feature = "integration-tests"))]
fn init(ctx: &Context, _awrgs: &[RedisString]) -> Status {
    let language_option = *language_option().lock(ctx);
    let low_accuracy_mode = *low_accuracy_mode().lock(ctx);

    init_detector(language_option, low_accuracy_mode);

    ctx.log_notice(
        format!(
            "Module initialized: language_option: {:?}, low_accuracy_mode: {}",
            language_option, low_accuracy_mode
        )
        .as_str(),
    );

    Status::Ok
}

// Set up the module
#[cfg(not(feature = "integration-tests"))]
redis_module! {
    name: "redislingua",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    init: init,
    commands: [
        ["lingua.detect", command::detect_language, "readonly", 0, 0, 0],
        ["lingua.get", command::detect_get, "readonly", 0, 0, 0],
        ["lingua.hget", command::detect_hget, "readonly", 0, 0, 0],
    ],
    configurations: [
        i64: [],
        string: [],
        bool: [
            ["low-accuracy-mode", low_accuracy_mode(), false, ConfigurationFlags::DEFAULT, None],
        ],
        enum: [
            ["language-option", language_option(), LanguageOption::AllLanguages, ConfigurationFlags::DEFAULT, None],
        ],
        module_args_as_configuration: true,
    ]
}
