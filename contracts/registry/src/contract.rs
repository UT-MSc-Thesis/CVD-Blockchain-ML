use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InfoResp, InstantiateMsg, QueryMsg, QueryWithPermit};
use crate::state::{OffspringInfo, Person, OFFSPRING, OWNER, PERSON_STORE};
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsg,
};
use secret_toolkit::viewing_key::{ViewingKey, ViewingKeyStore};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &msg.owner)?;
    OFFSPRING.save(
        deps.storage,
        &OffspringInfo {
            code_id: msg.offspring_id,
            code_hash: msg.offspring_hash,
        },
    )?;
    Ok(Response::new())
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Register { id, address, key } => {
            execute::register(deps, env, info, id, address, key)
        }
        ExecuteMsg::AddRecord {
            patient_id,
            record_id,
            record,
        } => execute::add_record(deps, patient_id, record_id, record),
    }
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Info { id, key } => {
            to_binary(&query::get_info(deps, id, key).unwrap()).map_err(Into::into)
        }
        QueryMsg::Records { id, page } => query::get_records(deps, id, page),
        QueryMsg::WithPermit { id, permit, query } => match query {
            QueryWithPermit::View => query::view_records(deps, id, permit),
            QueryWithPermit::Add => Ok(to_binary("").unwrap()),
        },
    }
}

pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        1 => reply::handle_instantiate_reply(deps, msg),
        id => Err(ContractError::UnexpectedReplyId { id }),
    }
}

mod execute {
    use super::*;
    use crate::msg::{AddRecordMsg, OffspringInstantiateMsg, Record};
    use secret_toolkit::utils::InitCallback;

    pub fn register(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        id: String,
        address: Addr,
        key: String,
    ) -> Result<Response, ContractError> {
        if OWNER.load(deps.storage).unwrap() != info.sender {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        let initmsg = OffspringInstantiateMsg {
            owner: address,
            owner_id: id,
            key: key,
        };

        let offspring = OFFSPRING.load(deps.storage).unwrap();

        let init_submsg = SubMsg::reply_always(
            initmsg.to_cosmos_msg(
                None,
                env.block.random.unwrap().to_string(),
                offspring.code_id,
                offspring.code_hash,
                None,
            )?,
            1,
        );

        Ok(Response::new().add_submessage(init_submsg))
    }

    pub fn add_record(
        deps: DepsMut,
        patient_id: String,
        record_id: String,
        record: Record,
    ) -> Result<Response, ContractError> {
        if !PERSON_STORE.contains(deps.storage, &patient_id) {
            return Err(ContractError::NonexistentUser { id: patient_id });
        }

        let offspring = OFFSPRING.load(deps.storage).unwrap();
        let person = PERSON_STORE.get(deps.storage, &patient_id).unwrap();

        let execute_msg = AddRecordMsg {
            id: record_id,
            title: record.title,
            description: record.description,
            data: record.data,
        };

        let processed_msg = execute_msg
            .clone()
            .into_cosmos_msg(person.contract_address.to_string(), offspring.code_hash)?;

        Ok(Response::new().add_message(processed_msg))
    }
}

mod query {
    use crate::msg::{OffspringQueryMsg, Record, RecordPermissions};
    use cosmwasm_std::{QueryRequest, WasmQuery};
    use secret_toolkit::permit::Permit;

    use super::*;

    pub fn get_info(deps: Deps, id: String, key: String) -> Result<InfoResp, ContractError> {
        if !PERSON_STORE.contains(deps.storage, &id) {
            return Err(ContractError::NonexistentUser { id: id });
        }

        let auth = ViewingKey::check(deps.storage, &id, &key);

        match auth.is_ok() {
            true => {
                let person = PERSON_STORE.get(deps.storage, &id).unwrap();
                let resp = InfoResp {
                    address: person.address,
                    contract_address: person.contract_address,
                };
                Ok(resp)
            }
            false => Err(ContractError::InvalidKey { key: key }),
        }
    }

    pub fn get_records(deps: Deps, id: String, page: u32) -> Result<Binary, ContractError> {
        if !PERSON_STORE.contains(deps.storage, &id) {
            return Err(ContractError::NonexistentUser { id: id });
        }

        let offspring = OFFSPRING.load(deps.storage).unwrap();
        let person = PERSON_STORE.get(deps.storage, &id).unwrap();

        let query_msg: OffspringQueryMsg = OffspringQueryMsg::Records { page: page };

        let query_response: Vec<(String, Record)> =
            deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: person.contract_address.to_string(),
                code_hash: offspring.code_hash,
                msg: to_binary(&query_msg)?,
            }))?;

        Ok(to_binary(&query_response).unwrap())
    }

    pub fn view_records(
        deps: Deps,
        id: String,
        permit: Permit<RecordPermissions>,
    ) -> Result<Binary, ContractError> {
        if !PERSON_STORE.contains(deps.storage, &id) {
            return Err(ContractError::NonexistentUser { id: id });
        }

        let offspring = OFFSPRING.load(deps.storage).unwrap();
        let person = PERSON_STORE.get(deps.storage, &id).unwrap();

        let query_msg: OffspringQueryMsg = OffspringQueryMsg::View { permit: permit };

        let query_response: Vec<(String, Record)> =
            deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: person.contract_address.to_string(),
                code_hash: offspring.code_hash,
                msg: to_binary(&query_msg)?,
            }))?;

        Ok(to_binary(&query_response).unwrap())
    }
}

mod reply {
    use super::*;
    use crate::msg::OffspringResp;
    use cosmwasm_std::{from_binary, SubMsgResult};

    pub fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
        match msg.result {
            SubMsgResult::Ok(s) => match s.data {
                Some(bin) => {
                    let resp: OffspringResp = from_binary(&bin)?;

                    PERSON_STORE.insert(
                        deps.storage,
                        &resp.owner_id,
                        &Person {
                            address: resp.owner_address,
                            contract_address: resp.offspring_address,
                        },
                    )?;

                    ViewingKey::set(deps.storage, &resp.owner_id, &resp.key);

                    Ok(Response::new())
                }
                None => Err(ContractError::OffspringInstantiationError {}),
            },
            SubMsgResult::Err(e) => Err(ContractError::CustomError { val: e }),
        }
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
                offspring_id: 1,
                offspring_hash: "".to_string(),
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
                offspring_id: 1,
                offspring_hash: "".to_string(),
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
                key: "".to_string(),
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
                key: "".to_string(),
            },
        )
        .unwrap();

        let resp = query(
            deps.as_ref(),
            env,
            QueryMsg::Info {
                id: sample_id.to_owned(),
                key: "".to_string(),
            },
        )
        .unwrap();
        let resp: InfoResp = from_binary(&resp).unwrap();

        assert_eq!(
            resp,
            InfoResp {
                address: sample_address,
                contract_address: Addr::unchecked("contract_address")
            }
        );
    }
}
