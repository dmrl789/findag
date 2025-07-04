// src/fix/schemas.rs

/// Example FIX Order Single message (SOH delimiter shown as \x01)
/// Format: 8=FIX.4.2|35=D|11=ORD12345|55=EUR/USD|54=1|38=1000000|44=1.1000|15=EUR|1=ALICE_ACCOUNT|10=123|
pub const EXAMPLE_FIX_ORDER: &str = "8=FIX.4.2\x0135=D\x0111=ORD12345\x0155=EUR/USD\x0154=1\x0138=1000000\x0144=1.1000\x0115=EUR\x011=ALICE_ACCOUNT\x0110=123\x01";

/// Example FIX Market Order (no price field)
pub const EXAMPLE_FIX_MARKET_ORDER: &str = "8=FIX.4.2\x0135=D\x0111=ORD67890\x0155=GBP/USD\x0154=2\x0138=500000\x0115=GBP\x011=BOB_ACCOUNT\x0110=456\x01";

/// Example FIX Execution Report (MsgType = 8)
pub const EXAMPLE_FIX_EXECUTION_REPORT: &str = "8=FIX.4.2\x0135=8\x0137=EXEC12345\x0111=ORD12345\x0155=EUR/USD\x0154=1\x0138=1000000\x0144=1.1000\x0115=EUR\x011=ALICE_ACCOUNT\x01150=2\x0139=2\x0110=789\x01";

/// Example FIX Order Cancel Request (MsgType = F)
pub const EXAMPLE_FIX_CANCEL_REQUEST: &str = "8=FIX.4.2\x0135=F\x0137=EXEC12345\x0141=ORD12345\x0111=CANCEL67890\x0155=EUR/USD\x0154=1\x0138=1000000\x0110=321\x01";

/// FIX field definitions for common fields
pub mod fields {
    pub const BEGIN_STRING: &str = "8";
    pub const MSG_TYPE: &str = "35";
    pub const CL_ORD_ID: &str = "11";
    pub const SYMBOL: &str = "55";
    pub const SIDE: &str = "54";
    pub const ORDER_QTY: &str = "38";
    pub const PRICE: &str = "44";
    pub const CURRENCY: &str = "15";
    pub const ACCOUNT: &str = "1";
    pub const CHECKSUM: &str = "10";
    pub const ORDER_ID: &str = "37";
    pub const ORIG_CL_ORD_ID: &str = "41";
    pub const EXEC_TYPE: &str = "150";
    pub const LEAVES_QTY: &str = "151";
    pub const CUM_QTY: &str = "14";
    pub const AVG_PX: &str = "6";
    pub const LAST_PX: &str = "31";
    pub const LAST_QTY: &str = "32";
    pub const ORDER_STATUS: &str = "39";
}

/// FIX message types
pub mod msg_types {
    pub const ORDER_SINGLE: &str = "D";
    pub const ORDER_CANCEL_REQUEST: &str = "F";
    pub const ORDER_CANCEL_REPLACE_REQUEST: &str = "G";
    pub const EXECUTION_REPORT: &str = "8";
    pub const ORDER_CANCEL_REJECT: &str = "9";
    pub const REJECT: &str = "3";
    pub const HEARTBEAT: &str = "0";
    pub const TEST_REQUEST: &str = "1";
    pub const RESEND_REQUEST: &str = "2";
    pub const LOGON: &str = "A";
    pub const LOGOUT: &str = "5";
}

/// FIX side values
pub mod sides {
    pub const BUY: &str = "1";
    pub const SELL: &str = "2";
    pub const BUY_MINUS: &str = "3";
    pub const SELL_PLUS: &str = "4";
    pub const SELL_SHORT: &str = "5";
    pub const SELL_SHORT_EXEMPT: &str = "6";
    pub const UNDISCLOSED: &str = "7";
    pub const CROSS: &str = "8";
    pub const CROSS_SHORT: &str = "9";
}

/// FIX execution types
pub mod exec_types {
    pub const NEW: &str = "0";
    pub const PARTIAL_FILL: &str = "1";
    pub const FILL: &str = "2";
    pub const DONE_FOR_DAY: &str = "3";
    pub const CANCELED: &str = "4";
    pub const REPLACE: &str = "5";
    pub const PENDING_CANCEL: &str = "6";
    pub const STOPPED: &str = "7";
    pub const REJECTED: &str = "8";
    pub const SUSPENDED: &str = "9";
    pub const PENDING_NEW: &str = "A";
    pub const CALCULATED: &str = "B";
    pub const EXPIRED: &str = "C";
    pub const RESTATED: &str = "D";
    pub const PENDING_REPLACE: &str = "E";
}

/// FIX order status values
pub mod order_status {
    pub const NEW: &str = "0";
    pub const PARTIALLY_FILLED: &str = "1";
    pub const FILLED: &str = "2";
    pub const DONE_FOR_DAY: &str = "3";
    pub const CANCELED: &str = "4";
    pub const REPLACED: &str = "5";
    pub const PENDING_CANCEL: &str = "6";
    pub const STOPPED: &str = "7";
    pub const REJECTED: &str = "8";
    pub const SUSPENDED: &str = "9";
    pub const PENDING_NEW: &str = "A";
    pub const CALCULATED: &str = "B";
    pub const EXPIRED: &str = "C";
    pub const ACCEPTED_FOR_BIDDING: &str = "D";
    pub const PENDING_REPLACE: &str = "E";
} 