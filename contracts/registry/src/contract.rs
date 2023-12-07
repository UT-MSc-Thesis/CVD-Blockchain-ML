use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InfoResp, InstantiateMsg, QueryMsg};
use crate::state::{Person, OWNER, PERSON_STORE};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &msg.owner)?;
    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { id, address } => execute::register(deps, info, id, address),
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Info { id } => to_binary(&query::get_info(deps, id).unwrap()),
    }
}

mod execute {
    use super::*;

    pub fn register(
        deps: DepsMut,
        info: MessageInfo,
        id: String,
        address: Addr,
    ) -> Result<Response, ContractError> {
        if OWNER.load(deps.storage).unwrap() != info.sender {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        PERSON_STORE.insert(deps.storage, &id, &Person { address: address })?;
        Ok(Response::new())
    }
}

mod query {
    use super::*;

    pub fn get_info(deps: Deps, id: String) -> StdResult<InfoResp> {
        let person = PERSON_STORE.get(deps.storage, &id).unwrap();
        let resp = InfoResp {
            address: person.address,
        };

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::from_binary;
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
            },
        )
        .unwrap();

        assert_eq!(0, resp.messages.len());
    }

    #[test]
    fn run_register_get_info() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let sample_id = "John Doe";
        let sample_address = Addr::unchecked("secret1");

        instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            InstantiateMsg {
                owner: Addr::unchecked("owner"),
            },
        )
        .unwrap();

        let err = execute(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            ExecuteMsg::Register {
                id: sample_id.to_owned(),
                address: sample_address.clone(),
            },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized {
                sender: Addr::unchecked("sender")
            },
            err
        );

        execute(
            deps.as_mut(),
            env.clone(),
            mock_info("owner", &[]),
            ExecuteMsg::Register {
                id: sample_id.to_owned(),
                address: sample_address.clone(),
            },
        )
        .unwrap();

        let resp = query(
            deps.as_ref(),
            env,
            QueryMsg::Info {
                id: sample_id.to_owned(),
            },
        )
        .unwrap();
        let resp: InfoResp = from_binary(&resp).unwrap();

        assert_eq!(
            resp,
            InfoResp {
                address: sample_address
            }
        );
    }
}
