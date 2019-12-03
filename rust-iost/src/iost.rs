
pub struct Options {
    pub server: String,
    pub account_name: String,
    pub gas_limit:   u64,
	pub gas_ratio:   u64,
	pub expiration: u64,
	pub amount_limit: String, // TODO
    pub check_result: bool,
    pub delay_second: i64,
    pub sign_algorithm: String,
	pub check_result_delay: u32,
    pub check_result_max_retry: i32,
}

pub struct IOST {
    pub options: Options,
	pub use_longest_chain: bool,
	pub verbose: bool,
	pub chain_id:  u32,
	pub rpc_connection:  String, //TODO
}

impl IOST {
    pub fn new(options: Options) -> Self {

        let o = Options {
            server: options.server,
            account_name: options.account_name,
            gas_limit: options.gas_limit,
            gas_ratio: options.gas_ratio,
            expiration: options.expiration,
            amount_limit: options.amount_limit, // TODO
            check_result: options.check_result,
            delay_second: options.delay_second,
            sign_algorithm: options.sign_algorithm,
            check_result_delay: options.check_result_delay,
            check_result_max_retry: options.check_result_max_retry,
        };
    
        let iost = IOST {
            options: o,
            use_longest_chain: false,
            verbose: false,
            chain_id: 1020,
            rpc_connection: String::from("")
        };
    
        iost
    }

    pub fn set_server(&mut self, server: String) -> &Self{
        self.options.server = server;
        self
    }

    pub fn set_account_name(&mut self, account_name: String) -> &Self{
        self.options.account_name = account_name;
        self
    }
    
}

