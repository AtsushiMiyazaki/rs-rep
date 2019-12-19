use bs58;
use ed25519_dalek::PublicKey;
use ed25519_dalek::SecretKey;
use ed25519_dalek::ExpandedSecretKey;
use ed25519_dalek::Signature;
use ed25519_dalek::Keypair;
use bytes::{BytesMut, BufMut};
use std::convert::TryInto;
use sha3::{Digest, Sha3_256};
use grpcio::{ChannelBuilder, EnvBuilder, Environment};
use log::*;
use std::sync::Arc;
use env_logger::*;
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

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

#[derive(Clone)]
pub struct Action {
    contract: String,
    action_name: String,
    data: String
}

#[derive(Clone)]
pub struct AmountLimit {
    token: String,
    value: String
}

#[derive(Clone)]
pub struct Sig {
    algorithm: String,
    public_key: PublicKey,
    signature: Signature,
}

pub struct RpcClient {
    client: grpcio::Client
}

pub struct Transaction {
    pub time: i64,
    pub expiration: i64,
    pub gas_ratio: i64,
    pub gas_limit: i64,
    pub delay: i64,
    pub chain_id: i32,
    pub actions: Vec<Action>,
    pub amount_limit: Vec<AmountLimit>,
    // pub publisher: String,
    // pub publisher_sigs: Vec<Sig>,
    pub signers: Vec<String>,
    pub signatures: Vec<String>,
}

pub struct Res {
    message: String
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    pub tx: Tx,
    // special fields
    pub unknown_fields: protobuf::UnknownFields,
    pub cached_size: protobuf::CachedSize,
}

impl Request {
    pub fn set_tx(&mut self, tx: Tx) -> &self {
        self.tx = tx;
        self
    }
}


#[derive(Clone,Default)]
pub struct Tx {
    pub time: i64,
    pub expiration: i64,
    pub gas_ratio: i64,
    pub gas_limit: i64,
    pub delay: i64,
    pub chain_id: i32,
    pub actions: Vec<Action>,
    pub amount_limit: Vec<AmountLimit>,
    pub publisher: String,
    pub publisher_sigs: Vec<Sig>,
    pub signers: Vec<String>,
    pub signatures: Vec<String>,
}

pub type PledgeAction = (u32, String, String);

pub struct IOST {
    pub options: Options,
	pub use_longest_chain: bool,
	pub verbose: bool,
	pub chain_id:  u32,
    pub rpc_connection:  String, //TODO
}

const METHOD_HELLOWORLD_CALL: grpcio::Method<Request, Res> = grpcio::Method {
    ty: grpcio::MethodType::Unary,
    name: "/sendTx",
    req_mar: grpcio::Marshaller { ser: grpcio::pb_ser, de: grpcio::pb_de },
    resp_mar: grpcio::Marshaller { ser: grpcio::pb_ser, de: grpcio::pb_de },
};

impl RpcClient {
    pub fn call(&self, req: &Request) -> grpcio::Result<Res> {
        self.call_opt(req, grpcio::CallOption::default())
    }

    pub fn call_opt(&self, req: &Request, opt: grpcio::CallOption) -> grpcio::Result<Res> {
        self.client.unary_call(&METHOD_HELLOWORLD_CALL, req, opt)
    }
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

    pub fn send(){
        std::env::set_var("RUST_LOG", "server=info");
        env_logger::init();
    
        let env = Arc::new(Environment::new(1));
        let ch = ChannelBuilder::new(env).connect("localhost:30001");

        let client = RpcClient{
            client: grpcio::Client::new(ch)
        }

        let t = self::IOST::create_tx_byte();
        let req = create_req(t);

        let reply = client.call(&req).expect("rpc");
        info!("Helloworld received: {}", reply.get_message());
    }

    pub fn create_req(tx: Tx) -> Request {
        let req = Request::default();
        req.set_tx(tx);
        req
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

    pub fn pub_key() -> PublicKey {
        let s = bs58::decode("2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1").into_vec().unwrap();

        let seckey: ExpandedSecretKey =  ExpandedSecretKey::from_bytes(&s).unwrap();
        let pubkey: PublicKey = (&seckey).into();
        pubkey
    }




    pub fn create_tx_byte() -> Tx {
        let s = bs58::decode("2yquS3ySrGWPEKywCPzX4RTJugqRh7kJSo5aehsLYPEWkUxBWA39oMrZ7ZxuM4fgyXYs2cPwh5n8aNNpH5x2VyK1").into_vec().unwrap();

        let seckey: ExpandedSecretKey =  ExpandedSecretKey::from_bytes(&s).unwrap();
        let pubkey: PublicKey = (&seckey).into();

        let actions = vec!( Action {
                contract: String::from("token.iost"),
                action_name: String::from("transfer"),
                data: String::from("[\"iost\", \"testaccount\", \"anothertest\", \"100\", \"this is an example transfer\"]")
            });
        let amount_limit = vec!( AmountLimit {
            token: String::from("*"),
            value: String::from("unlimited")
        });

        // let sig = vec!( Sig{
        //     algorithm: String::from("ED25519"),
        //     signature: String::from(""),
        //     public_key: String::from("")
        // });

        let tx = Transaction {
            time: 1544709662543340000,
            expiration: 1544709692318715000,
            gas_ratio: 1,
            gas_limit: 500000,
            delay: 0,
            chain_id: 1024,
            signers: [].to_vec(),
            actions,
            amount_limit,
            signatures: [].to_vec()
        };

        let tx_bytes = self::IOST::_bytes(tx);
        let hashed_bytes = self::IOST::_hash(tx_bytes);

        let s = seckey.sign(&hashed_bytes, &pubkey);
        
        let tx = Tx {
            time: 1544709662543340000,
            expiration: 1544709692318715000,
            gas_ratio: 1,
            gas_limit: 500000,
            delay: 0,
            chain_id: 1024,
            signers: [].to_vec(),
            actions,
            amount_limit,
            signatures: [].to_vec(),
            publisher: String::from("admin"),
            publisher_sigs: vec!(Sig {
                algorithm: "ED25519".to_string(),
                public_key: pubkey,
                signature: s 
            })
        };
        tx
    }

    fn _hash( b: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        
        hasher.input(b"hello");
        
        let result = hasher.result();
        result.to_vec()
    }

    fn _bytes(tx: Transaction) -> Vec<u8>{
        let mut b = BytesMut::with_capacity(1024);
        // let gas_ratio: i64 = tx.gas_ratio.round() as i64;
        // let gas_limit: i64 = tx.gas_ratio.round() as i64;
        b.put_i64_be(tx.time);
        b.put_i64_be(tx.expiration);
        b.put_i64_be( tx.gas_ratio);
        b.put_i64_be( tx.gas_limit);
        b.put_i64_be(tx.delay);
        b.put_i32_be(tx.chain_id);
        b.put_i32_be(0);

        let signers_len = tx.signers.len().try_into().unwrap();

        b.put_i32_be(signers_len);
        for i in 0 .. tx.signers.len() {
            b.put(tx.signers[i].as_bytes());
        };

        let actions_len = tx.actions.len().try_into().unwrap();
        b.put_i32_be(actions_len);
        for i in 0 .. tx.actions.len(){
            b.put(tx.actions[i].contract.as_bytes());
            b.put(tx.actions[i].action_name.as_bytes());
            b.put(tx.actions[i].data.as_bytes());
        };

        let amount_len = tx.amount_limit.len().try_into().unwrap();
        b.put_i32_be(amount_len);
        for i in 0 .. tx.amount_limit.len(){
            b.put(tx.amount_limit[i].token.as_bytes());
            b.put(tx.amount_limit[i].value.as_bytes());
        };

        b.to_vec()
    }
}



