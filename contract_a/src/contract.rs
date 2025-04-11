use cosmwasm_std::{
    entry_point, to_json_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, WasmMsg,
};

use crate::msg::{ExecuteMsg, InstantiateMsg};
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}
#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SendMessage { message, target } => {
            let call_msg = ExecuteMsg::ReceiveCall { message };
            let wasm_msg = WasmMsg::Execute {
                contract_addr: target,
                msg: to_json_binary(&call_msg)?,
                funds: vec![],
            };
            Ok(Response::new().add_message(CosmosMsg::Wasm(wasm_msg)))
        }
        ExecuteMsg::ReceiveCall { message } => todo!(),
    }
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: Binary) -> StdResult<Binary> {
    Ok(Binary::from(b"{}".as_ref()))
}
