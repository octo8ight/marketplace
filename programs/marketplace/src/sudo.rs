#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::error::ContractError;
use crate::helpers::ExpiryRange;
use crate::msg::SudoMsg;
use crate::state::{ASK_HOOKS, BID_HOOKS, SALE_HOOKS, SUDO_PARAMS};
use cosmwasm_std::{Addr, Decimal, DepsMut, Env, Uint128};
use cw_utils::Duration;
use sg_std::Response;

// bps fee can not exceed 100%
const MAX_FEE_BPS: u64 = 10000;

pub struct ParamInfo {
    trading_fee_bps: Option<u64>,
    ask_expiry: Option<ExpiryRange>,
    bid_expiry: Option<ExpiryRange>,
    operators: Option<Vec<String>>,
    max_finders_fee_bps: Option<u64>,
    min_price: Option<Uint128>,
    stale_bid_duration: Option<u64>,
    bid_removal_reward_bps: Option<u64>,
    listing_fee: Option<Uint128>,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        SudoMsg::UpdateParams {
            trading_fee_bps,
            ask_expiry,
            bid_expiry,
            operators,
            max_finders_fee_bps,
            min_price,
            stale_bid_duration,
            bid_removal_reward_bps,
            listing_fee,
        } => sudo_update_params(
            deps,
            env,
            ParamInfo {
                trading_fee_bps,
                ask_expiry,
                bid_expiry,
                operators,
                max_finders_fee_bps,
                min_price,
                stale_bid_duration,
                bid_removal_reward_bps,
                listing_fee,
            },
        ),
        SudoMsg::AddOperator { operator } => sudo_add_operator(deps, api.addr_validate(&operator)?),
        SudoMsg::RemoveOperator { operator } => {
            sudo_remove_operator(deps, api.addr_validate(&operator)?)
        }
        SudoMsg::AddSaleHook { hook } => sudo_add_sale_hook(deps, api.addr_validate(&hook)?),
        SudoMsg::AddAskHook { hook } => sudo_add_ask_hook(deps, env, api.addr_validate(&hook)?),
        SudoMsg::AddBidHook { hook } => sudo_add_bid_hook(deps, env, api.addr_validate(&hook)?),
        SudoMsg::RemoveSaleHook { hook } => sudo_remove_sale_hook(deps, api.addr_validate(&hook)?),
        SudoMsg::RemoveAskHook { hook } => sudo_remove_ask_hook(deps, api.addr_validate(&hook)?),
        SudoMsg::RemoveBidHook { hook } => sudo_remove_bid_hook(deps, api.addr_validate(&hook)?),
    }
}
