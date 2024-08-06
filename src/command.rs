use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue};

use crate::detector::run_language_detection;

/// LINGUA.DETECT <text>
///
/// Detects the language of the given text.
pub(crate) fn detect_language(_ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let value = args.next_arg()?;

    match run_language_detection(&value) {
        Some(lang) => Ok(lang.into()),
        None => Ok(RedisValue::Null),
    }
}

/// LINGUA.GET <key>
///
/// Detects the language of the value stored at the given key.
pub(crate) fn detect_get(ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);
    let key_name = args.next_arg()?;

    let key = ctx.open_key(&key_name);
    match key
        .read()?
        .map(|v| RedisString::create_from_slice(std::ptr::null_mut(), v))
    {
        Some(v) => match run_language_detection(&v) {
            Some(lang) => Ok(lang.into()),
            None => Ok(RedisValue::Null),
        },
        None => Ok(RedisValue::Null),
    }
}

/// LINGUA.HGET <key> <field>
///
/// Detects the language of the value stored at the given field in the hash.
pub(crate) fn detect_hget(ctx: &Context, args: Vec<RedisString>) -> RedisResult<RedisValue> {
    if args.len() < 3 {
        return Err(RedisError::WrongArity);
    }

    let mut args = args.into_iter().skip(1);

    let hash_name = args.next_arg()?;
    let hash = ctx.open_key(&hash_name);

    let hash_key_name = args.next_arg()?;
    match hash.hash_get(hash_key_name.try_as_str()?)? {
        Some(v) => match run_language_detection(&v) {
            Some(lang) => Ok(lang.into()),
            None => Ok(RedisValue::Null),
        },
        None => Ok(RedisValue::Null),
    }
}
