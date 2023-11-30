use cosmwasm_std::{
    Binary, ContractResult, CosmosMsg, Empty, QuerierWrapper, QueryRequest, StdError, StdResult,
    SystemResult, Timestamp,
};
use prost::bytes::Bytes;
use prost_types::Timestamp as TimestampGen;
use serde_json_wasm::to_vec;

pub(crate) fn make_stargate_query<Req, Res>(
    querier: QuerierWrapper,
    req: Req,
    path: &str,
) -> StdResult<Res>
where
    Req: prost::Message,
    Res: prost::Message + Default,
{
    let raw = to_vec::<QueryRequest<Empty>>(&QueryRequest::Stargate {
        path: path.to_string(),
        data: req.encode_to_vec().into(),
    })
    .map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;

    match querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Res::decode(Bytes::copy_from_slice(
            Binary::from_base64(&value.to_base64())?.as_slice(),
        ))
        .map_err(|e| StdError::generic_err(e.to_string())),
    }
}

pub(crate) fn create_stargate_msg<Req>(req: Req, path: &str) -> CosmosMsg
where
    Req: prost::Message,
{
    cosmwasm_std::CosmosMsg::Stargate {
        type_url: path.to_string(),
        value: Binary::from(req.encode_to_vec()),
    }
}

pub(crate) fn convert_timestamp(timestamp: Timestamp) -> TimestampGen {
    TimestampGen {
        seconds: i64::try_from(timestamp.seconds()).unwrap(),
        nanos: i32::try_from(timestamp.subsec_nanos()).unwrap(),
    }
}
