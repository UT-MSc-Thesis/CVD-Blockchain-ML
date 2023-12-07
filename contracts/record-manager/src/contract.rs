use crate::error::ContractError;
use crate::msg::{CallbackInfo, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::OWNER;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &msg.owner)?;

    let callback_info = CallbackInfo {
        offspring_address: env.contract.address,
        owner_address: msg.owner,
        owner_id: msg.owner_id,
    };

    Ok(Response::new().set_data(to_binary(&callback_info)?))
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Ok(to_binary(&Addr::unchecked("someaddress")).unwrap())
}

mod execute {}

mod query {}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    #[test]
    fn run_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let resp = instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            InstantiateMsg {
                owner: Addr::unchecked("owner"),
                owner_id: "John Doe".to_string(),
            },
        )
        .unwrap();

        assert_eq!(0, resp.messages.len());
    }
}