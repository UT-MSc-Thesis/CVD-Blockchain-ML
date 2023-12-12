use crate::error::ContractError;
use crate::msg::{CallbackInfo, ExecuteMsg, InstantiateMsg, QueryMsg, RecordPermissions};
use crate::state::{Record, RECORD_STORE};
use crate::state::{OWNER, REGISTRY};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &msg.owner)?;
    REGISTRY.save(deps.storage, &info.sender)?;

    let callback_info = CallbackInfo {
        offspring_address: env.contract.address,
        owner_address: msg.owner,
        owner_id: msg.owner_id,
        key: msg.key,
    };

    Ok(Response::new().set_data(to_binary(&callback_info)?))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    if REGISTRY.load(deps.storage).unwrap() != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
        });
    }

    match msg {
        ExecuteMsg::AddRecord {
            id,
            title,
            description,
            data,
        } => execute::add_record(deps, env, id, title, description, data),
    }
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Records { page } => query::get_records(deps, page),
        QueryMsg::View { permit } => {
            secret_toolkit::permit::validate(
                deps,
                "revoked_permits",
                &permit,
                env.contract.address.to_string(),
                None,
            )?;

            if !permit.check_permission(&RecordPermissions::View) {
                // return Err(ContractError::InvalidPermit);
                return Err(StdError::generic_err("Invalid Permit"));
            }

            query::get_records(deps, 0)
        }
    }
}

mod execute {
    use super::*;

    pub fn add_record(
        deps: DepsMut,
        env: Env,
        id: String,
        title: String,
        description: String,
        data: String,
    ) -> Result<Response, ContractError> {
        RECORD_STORE.insert(
            deps.storage,
            &id,
            &Record {
                title: title,
                timestamp: env.block.time,
                description: description,
                data: data,
            },
        )?;

        Ok(Response::new())
    }
}

mod query {
    use super::*;

    pub fn get_records(deps: Deps, page: u32) -> StdResult<Binary> {
        let records = RECORD_STORE.paging(deps.storage, page, 5).unwrap();
        Ok(to_binary(&records).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        Addr,
    };

    use super::*;

    #[test]
    fn run_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let resp = instantiate(
            deps.as_mut(),
            env,
            mock_info("sender", &[]),
            InstantiateMsg {
                owner: Addr::unchecked("owner"),
                owner_id: "John Doe".to_string(),
                key: "".to_string(),
            },
        )
        .unwrap();

        assert_eq!(0, resp.messages.len());
    }

    #[test]
    fn run_add_record_unauthorized() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("registry", &[]),
            InstantiateMsg {
                owner: Addr::unchecked("owner"),
                owner_id: "Alice".to_string(),
                key: "password".to_string(),
            },
        )
        .unwrap();

        let err = execute(
            deps.as_mut(),
            env,
            mock_info("sender", &[]),
            ExecuteMsg::AddRecord {
                id: "id".to_string(),
                title: "title".to_string(),
                description: "decription".to_string(),
                data: "data".to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(
            err,
            ContractError::Unauthorized {
                sender: Addr::unchecked("sender")
            }
        );
    }

    #[test]
    fn run_add_record() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("registry", &[]),
            InstantiateMsg {
                owner: Addr::unchecked("owner"),
                owner_id: "Alice".to_string(),
                key: "password".to_string(),
            },
        )
        .unwrap();

        execute(
            deps.as_mut(),
            env.clone(),
            mock_info("registry", &[]),
            ExecuteMsg::AddRecord {
                id: "id".to_string(),
                title: "title".to_string(),
                description: "description".to_string(),
                data: "data".to_string(),
            },
        )
        .unwrap();

        let resp = query(deps.as_ref(), env.clone(), QueryMsg::Records { page: 0 }).unwrap();
        let resp: Vec<(String, Record)> = from_binary(&resp).unwrap();

        assert_eq!(
            vec![(
                "id".to_string(),
                Record {
                    title: "title".to_string(),
                    timestamp: env.block.time,
                    description: "description".to_string(),
                    data: "data".to_string()
                }
            )],
            resp
        );
    }
}
