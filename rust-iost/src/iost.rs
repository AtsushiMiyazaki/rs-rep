use bs58;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use ed25519_dalek::Signature;
use ed25519_dalek::Keypair;

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

pub struct Action<T> {
    contract: String,
    action_name: String,
    data: T
}

pub struct AmountLimit {
    token: String,
    value: f64
}

pub struct Sig {
    algorithm: String,
    signature: String,
    public_key: String,
}

pub struct Transaction<T> {
    pub time: i64,
    pub expiration: i64,
    pub gas_ratio: f64,
    pub gas_limit: f64,
    pub delay: i64,
    pub chain_id: u32,
    pub actions: Vec<Action<T>>,
    pub amount_limit: Vec<AmountLimit>,
    pub publisher: String,
    pub publisher_sigs: Vec<Sig>
}

pub type PledgeAction = (u32, String, String);

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

    pub fn set_chain_id(&mut self, chain_id: u32) -> &Self{
        self.chain_id = chain_id;
        self
    }

    pub fn set_tx_options(&mut self, gas_limit: u64, gas_ratio: u64, expiration: u64) -> &Self{
        self.options.gas_limit = gas_limit;
        self.options.gas_ratio = gas_ratio;
        self.options.expiration = expiration;
        self
    }
    
    pub fn set_sign_algorithm(&mut self, sign_algorithm: String)-> Result<&Self, ()>{
        if sign_algorithm != String::from("ed25519") ||
           sign_algorithm != String::from("SECP256K1")
        {
            return Err(())
        }

        self.options.sign_algorithm = sign_algorithm;
        Ok(self)
    }

    pub fn set_use_longest_chain(&mut self, use_longest_chain : bool) -> &Self{
        self.use_longest_chain = use_longest_chain;
        self
    }


    pub fn create_tx_byte(){
        let s = bs58::decode("2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1").into_vec().unwrap();

        let seckey: SecretKey =  SecretKey::from_bytes(&s).unwrap();
        let pubkey: PublicKey = (&seckey).into();
        let keypair =  Keypair {
            secret: seckey,
            public: pubkey
        };

        let actions = vec!( Action {
                contract: String::from("gas.iost"),
                action_name: String::from("pledge"),
                data: (100, String::from("admin"), String::from("admin"))
            });
        let amount_limit = vec!( AmountLimit {
            token: String::from("iost"),
            value: 100.0
        });

        let sig = vec!( Sig{
            algorithm: String::from("ED25519"),
            signature: String::from(""),
            public_key: String::from("")
        });

        let tx = Transaction {
            time: 1575804827,
            expiration: 1585804827,
            gas_ratio: 1.0,
            gas_limit: 100000.0,
            delay: 0,
            chain_id: 1020,
            actions,
            amount_limit,
            publisher: String::from("admin"),
            publisher_sigs: sig
        };
    }

    fn _bytes(&tx: Transaction<T>) -> Vec<u8>{

    };
}



