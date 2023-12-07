use crate::msg::{ExecuteMsg, InfoResp, QueryMsg};
use crate::state::{Person, PERSON_STORE};
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Register { id, address } => execute::register(deps, id, address),
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Info { id } => to_binary(&query::get_info(deps, id).unwrap()),
    }
}

mod execute {
    use super::*;

    pub fn register(deps: DepsMut, id: String, address: Addr) -> StdResult<Response> {
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
            Empty {},
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

        execute(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
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
