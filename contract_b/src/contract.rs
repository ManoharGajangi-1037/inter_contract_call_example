use cosmwasm_std::{
    entry_point, Deps, DepsMut, Env, MessageInfo, StdResult, Response, StdError, Binary, to_binary
};

use crate::msg::{ExecuteMsg, InstantiateMsg, MessagesResponse, QueryMsg};
use crate::state::{MESSAGES, AUTHORIZED_CONTRACT};
use crate::auth_sender;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Save the deployer as authorized for now (can be changed)
    AUTHORIZED_CONTRACT.save(deps.storage, &info.sender)?;
    MESSAGES.save(deps.storage, &vec![])?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::ReceiveCall { message } => {
            let authorized = AUTHORIZED_CONTRACT.load(deps.storage)?;
            auth_sender!(info, authorized);

            let mut msgs = MESSAGES.load(deps.storage)?;
            msgs.push(message);
            MESSAGES.save(deps.storage, &msgs)?;

            Ok(Response::new().add_attribute("method", "receive_call"))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessages {} => {
            let msgs = MESSAGES.load(deps.storage)?;
            to_binary(&MessagesResponse { messages: msgs })
        }
    }
}
