#[macro_export]
macro_rules! auth_sender {
    ($info:expr, $authorized:expr) => {
        if $info.sender != $authorized {
            return Err(cosmwasm_std::StdError::generic_err("unauthorized"));
        }
    };
}
