use cosmwasm_std::{CosmosMsg, CustomMsg};

use crate::proto_types::neutron::dex::{
    MsgCancelLimitOrder, MsgDeposit, MsgMultiHopSwap, MsgPlaceLimitOrder,
    MsgWithdrawFilledLimitOrder, MsgWithdrawal,
};
use crate::stargate::aux::create_stargate_msg;
use crate::stargate::dex::types::{
    CancelLimitOrderRequest, DepositRequest, MultiHopSwapRequest, PlaceLimitOrderRequest,
    WithdrawFilledLimitOrderRequest, WithdrawalRequest,
};

const DEPOSIT_MSG_PATH: &str = "/neutron.dex.MsgDeposit";
const WITHDRAWAL_MSG_PATH: &str = "/neutron.dex.MsgWithdrawal";
const PLACE_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgPlaceLimitOrder";
const WITHDRAW_FILLED_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgWithdrawFilledLimitOrder";
const CANCEL_LIMIT_ORDER_MSG_PATH: &str = "/neutron.dex.MsgCancelLimitOrder";
const MULTI_HOP_SWAP_MSG_PATH: &str = "/neutron.dex.MsgMultiHopSwap";

/// Provides liquidity to a specific trading pair by depositing tokens at a specific price into one
/// or both sides of the pair in “a liquidity pool”.
pub fn msg_deposit<T: CustomMsg>(req: DepositRequest) -> CosmosMsg<T> {
    create_stargate_msg(DEPOSIT_MSG_PATH, MsgDeposit::from(req))
}

/// Redeems PoolShares for the user’s pro-rata portion of tokens within a liquidity pool. When
/// withdrawing from a pool they will receive token_a and token_b in the same ratio as what is
/// currently present in the pool.
pub fn msg_withdrawal<T: CustomMsg>(req: WithdrawalRequest) -> CosmosMsg<T> {
    create_stargate_msg(WITHDRAWAL_MSG_PATH, MsgWithdrawal::from(req))
}

/// Provides new liquidity to the dex that can be swapped through by other traders.
pub fn msg_place_limit_order<T: CustomMsg>(req: PlaceLimitOrderRequest) -> CosmosMsg<T> {
    create_stargate_msg(PLACE_LIMIT_ORDER_MSG_PATH, MsgPlaceLimitOrder::from(req))
}

/// Withdraws all available credits from an either partially or entirely fulfilled limit order.
pub fn msg_withdraw_filled_limit_order<T: CustomMsg>(
    req: WithdrawFilledLimitOrderRequest,
) -> CosmosMsg<T> {
    create_stargate_msg(
        WITHDRAW_FILLED_LIMIT_ORDER_MSG_PATH,
        MsgWithdrawFilledLimitOrder::from(req),
    )
}

/// Cancels a standard taker limit order (Good-til-cancelled | Good-til-time) if it has not been
/// completely filled. Once a limit order is canceled any remaining “TokenIn” liquidity is returned
/// to the user.
///
/// NOTE: Cancelling a partially filled limit order does not withdraw the traded portion. A separate
/// call must be made to `WithdrawFilledLimitOrder` to withdraw any proceeds from the limit order.
pub fn msg_cancel_limit_order<T: CustomMsg>(req: CancelLimitOrderRequest) -> CosmosMsg<T> {
    create_stargate_msg(CANCEL_LIMIT_ORDER_MSG_PATH, MsgCancelLimitOrder::from(req))
}

/// Swaps by routing through a series of pools to achieve better prices.
pub fn msg_multi_hop_swap<T: CustomMsg>(req: MultiHopSwapRequest) -> CosmosMsg<T> {
    create_stargate_msg(MULTI_HOP_SWAP_MSG_PATH, MsgMultiHopSwap::from(req))
}
