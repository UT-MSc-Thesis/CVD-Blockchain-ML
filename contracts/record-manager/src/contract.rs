use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InfoResp, InstantiateMsg, QueryMsg};
use crate::state::{Person, OWNER, PERSON_STORE};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // OWNER.save(deps.storage, &msg.owner)?;
    Ok(Response::new())
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
    // match msg {
    //     ExecuteMsg::Register { id, address } => execute::register(deps, info, id, address),
    // }
}

pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Ok(to_binary("data").unwrap())
    // match msg {
    //     QueryMsg::Info { id } => to_binary(&query::get_info(deps, id).unwrap()),
    // }
}

mod execute {
    // use super::*;
}

mod query {
    // use super::*;
}

#[cfg(test)]
mod tests {
    // use cosmwasm_std::from_binary;
    // use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    // use super::*;

    // #[test]
    // fn run_instantiate() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();

    //     let resp = instantiate(
    //         deps.as_mut(),
    //         env.clone(),
    //         mock_info("sender", &[]),
    //         InstantiateMsg {
    //             owner: Addr::unchecked("owner"),
    //         },
    //     )
    //     .unwrap();

    //     assert_eq!(0, resp.messages.len());
    // }

    // #[test]
    // fn run_register_get_info() {
    //     let mut deps = mock_dependencies();
    //     let env = mock_env();
    //     let sample_id = "John Doe";
    //     let sample_address = Addr::unchecked("secret1");

    //     instantiate(
    //         deps.as_mut(),
    //         env.clone(),
    //         mock_info("sender", &[]),
    //         InstantiateMsg {
    //             owner: Addr::unchecked("owner"),
    //         },
    //     )
    //     .unwrap();

    //     let err = execute(
    //         deps.as_mut(),
    //         env.clone(),
    //         mock_info("sender", &[]),
    //         ExecuteMsg::Register {
    //             id: sample_id.to_owned(),
    //             address: sample_address.clone(),
    //         },
    //     )
    //     .unwrap_err();

    //     assert_eq!(
    //         ContractError::Unauthorized {
    //             sender: Addr::unchecked("sender")
    //         },
    //         err
    //     );

    //     execute(
    //         deps.as_mut(),
    //         env.clone(),
    //         mock_info("owner", &[]),
    //         ExecuteMsg::Register {
    //             id: sample_id.to_owned(),
    //             address: sample_address.clone(),
    //         },
    //     )
    //     .unwrap();

    //     let resp = query(
    //         deps.as_ref(),
    //         env,
    //         QueryMsg::Info {
    //             id: sample_id.to_owned(),
    //         },
    //     )
    //     .unwrap();
    //     let resp: InfoResp = from_binary(&resp).unwrap();

    //     assert_eq!(
    //         resp,
    //         InfoResp {
    //             address: sample_address
    //         }
    //     );
    // }
}
