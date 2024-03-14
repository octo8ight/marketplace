use crate::msg::ExecuteMsg;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Api, BlockInfo, StdError, StdResult, Timestamp, WasmMsg};
use sg_std::CosmosMsg;
use thiserror::Error;

/// MarketplaceContract is a wrapper around Addr that provides a lot of helpers
#[cw_serde]
pub struct MarketplaceContract(pub Addr);

impl MarketplaceContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

pub fn map_validate(api: &dyn Api, addresses: &[String]) -> StdResult<Vec<Addr>> {
    let mut validated_addresses = addresses
        .iter()
        .map(|addr| api.addr_validate(addr))
        .collect::<StdResult<Vec<_>>>()?;
    validated_addresses.sort();
    validated_addresses.dedup();
    Ok(validated_addresses)
}

#[derive(Error, Debug, PartialEq)]
pub enum ExpiryRangeError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid expiration range")]
    InvalidExpirationRange {},

    #[error("Expiry min > max")]
    InvalidExpiry {},
}
