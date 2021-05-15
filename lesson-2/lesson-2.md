# lesson 2 substrate & polkadot简介

## ipfs eos btc eth zcash polkadot
可扩展性 去中心化 安全性
### ipfs
* 安全性不够
* 去中心化存储，仅检查hash值
* 交易顺序，节点状态无严格限制，无法解决双花
* 系统一致性，无法保证
### eos
共识算法：dpos，要求21个共识节点，去中心化不够

### btc eth
* pow 去中心化足够，足够安全
* 可扩展性弱，每个节点做着同样的事情

### zcash
* 隐私链

### polkadot
* 跨链互通
* 中介链（relay chain）validators(区块验证节点)
* 平行链（Parachain) Collators(区块整理节点) bridge（桥）
* relaychain -> 1级 parachain -> 2级 parachain ...
* polkadot作为联盟链
> * 核心链（relaychain）
> * 政务链（parachain）、人才链（parachain）、供应链（parachain）、银行链（parachain）、资产管理链（parachain）、征信链（parachain）

## substrate
### 区块链节点需要：
* 核心组件
> * 数据库
> * 点对点网络
> * 共识算法
> * 交易处理
> * 状态转换函数（Runtime）
> * 加密算法
> * 序列化
> * 其他特别的函数：零知识证明，分片，侧链，状态通道。。。
* 治理、升级模型
* 智能合约（dapp)

### 给开发者的灵活度
* 公链
> dapp、出块策略配置
* 联盟链
> 共识算法选择、智能合约、数据库选择、出块策略配置
* substrate
> 全部都可以自由定制
> 只要实现对应接口的功能即可

### substrate优点
* 可扩展性
* 模块化
* 开源
* 自主可控

### substrate包含什么？
* 核心模块
> * 数据库
> * 交易队列
> * 命令行界面
> * 公/密钥生成
> * RPC

* 基本逻辑
> * 数据结构
> * 结算
> * 时间戳

* p2p网络
> * 网络节点管理
> * 私讯协议集成
> * 分布式hash表

* 共识机制
> * 抵押
> * Babe
> * Grandpa
> * 区块落地追踪

* 链上治理
> * 民主
> * 投票
> * 议会
> * 国库

### polkadot: substrate之上建立的模块
* 平行链 parachains
* 区块整理 collators
* 跨链通讯协议 cross chain mesage passing
* 私讯协议 gossip protocol
* 持续可用存储 persistent availability store
* 平行线程 parathreads
* 众筹模块 crowd funding
* 赔偿 claims
* 拍卖 auctions
* 公正 registar

### The Substrate Runtime
* Runtime时区块链的链上运行逻辑的集合。也就是状态转换函数。
* Runtime组件（目前支持有40多个了）：
>* 资产
>* 共识
>* 余额
>* collective
>* 合约
>* 治理
>* 选举
>* grandpa
>* 账户
>* 块确认
>* indices
>* 会员
>* offences
>* session
>* 抵押
>* 超级权限
>* system
>* 时间戳
>* 国库
>* more
* 有自定义runtime的范式

### Runtime升级的治理Governing Runtime Upgrades
* Runtime代码可以通过链上治理访问
* Sudo模块:一键升级
* Democracy模块：公投升级
* 自定义的模块和逻辑
* Runtime升级是可选的
* 一键链上升级——永不分叉
> Native client environment -> entry apis -> Native Runtime和链上Runtime版本是否一致？Native runtime(from client):Wasm runtime(from chain) WebAssambly Interpreter -> Merklised storage database
* 为什么需要链上升级
>* 修复重要的安全漏洞
>* 改变核心规则
>* 添加新功能
>* 修复链上状态
* 硬分叉需要的协作成本极高，且易升级失败
* 没有明确的治理策略和升级时间点
* note:使用Wasm，升级过程无需节点直接参与。如果不使用Wasm，整个网络都需要执行升级操作。

### Substrate与企业系统无缝集成
使用区块链成为解决方案的一部分
* 使用内置链下工作机，提供与SAP,Oracle,SQL服务器以及更多其他企业系统的无缝集成支持。
> SAP and other SAP products and services mentioned herein as well as their respective logos are trademarks or registered trademarks of SAP SE(or an SAP affiliate company) in Germany and other countries.
> SQL Server and its logo is registered trademark of Microsoft Corporation.
* 支持集成Trusted Execution Environments(TEEs)。
* 内置与区块链预言机的双向集成。
* 使用JSON-RPC代理的中间件集成

### substrate安利
Substrate是公链技术、生态和联盟链之间的桥梁
通过Substrate分享最先进的区块链技术成果

### substrate资料
* 官方文档https://substrate.io
>* Tutorials
>* Videos
>* Playground：在线运行
>* Community
>* Recipes:碎片化小资料
>* Docs：入门递进文档
>* Samples
* 中文学习资料
>* substrate.io
>* Bilibili: https://space.bilibili.com/67358318
>* 知乎： https://zhuanlian.zhihu.com/substrate
>* 知乎： https://zhuanlian.zhihu.com/v2web3
>* 微信公众号： polkadot中文平台、Polkadotword、Polkabase

## 运行substrate节点
* 单节点（开发dev）
> ./target/release/substrate purge-chain --dev
> ./target/release/substrate --dev
* 多节点（本地链local）
> ./target/release/substrate --alice --chain local --base-path /tmp/alice
> ./target/release/substrate --bob --chain local --base-path /tmp/bob
* polkadot.js.org查看自己本地链

## Substrate Node Template
### 模块定义概览
```rust
use support::{decl_module, decl_storage, decl_event,*};
pub trait Trait: system::Trait {...}
// frame v1
decl_storate!{...} // frame v2 pallet::storage
decl_event!{...} // frame v2 pallet::event
decl_error!{...} // frame v2 pallet::error
decl_module!{...}
impl<T: Trait> Module<T> {...}
```

### 引入和定义关联类型
```rust
// 自定义trait名字为Trait，继承自system pallet的Trait
pub trait Trait: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event;
}
//...and it inherits from system::Trait:
// From `system` pallet
// 自定义关联类型和其他pallet交互，给自定义类型trait约束，比如可加、可减
// 比如拍卖系统，可以不直接调用系统的transfer，而是定义自己的代币转账
pub trait Trait: 'static + Eq + Clone {
    type Origin: ...
    type Call: ...
    type Index: ...
    Type BlockNumber: ...
}
```

### 定义存储
```rust
// 宏里面定义的格式不是严格的rust语法
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        // Here we are declaring a StorageValue, `Something` as an Option<u32>
        // `get(fn something)` defines a getter function
        // Getter called with `Self::thing()`
        Something get(fn something): Option<u32>;
        // Here we are declaring a StorageMap `SomeMap` from an AccountId to 
        // a Hash.
        // Getter called with `Self::some_map(account_id)`
        SomeMap get(fn some_map): map hasher(identity) T::AccountId => u32;
    }
}
```

### 定义可调用函数
```rust
// 宏里面定义的格式不是严格的rust语法
decl_module!{
    pub struct Module<T: Trait> for enum Call where origin: T::origin {
        fn deposit_event<T>() = default; // The default deposit_event definition
        // 用户可以调用的方法
        pub fn do_something(origin, something: u32) -> Result {
            let sender = ensure_signed(origin)?; // Check for transaction
            <Something::put(something); // Put a value into a StorageValue
            // 调用成功会发送event事件SomethingStored
            Self::deposit_event(RawEvent::SomethingStored(something, who)); // Emit Event
            Ok(()) // Return Ok at the end of a function
        }
    }
}
```

### 定义公共和私有函数
```rust
impl<T: Trait> Module<T>{
    // 如果定义为pub其他模块也可以调用
    fn mint(to: T::AccountId, id: T::Hash) -> Result {...}
    fn transfer(from: T::AccountId, to: T::AccountId, id: T::Hash) -> Result {...}
}
```

### 定义事件
```rust
// 宏里面定义的格式不是严格的rust语法
decl_event!{
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
    /// Event `SomethingStored` is declared with a parameter of the type `u32` and `AccountId`
    SomethingStored(u32, AccountId),
    }
}
```

### 代码
* https://github.com/SubstrateCourse/substrate-node-template
* node
> src
>* chain_spec.rs 创世块配置
>* cli.rs 命令行语句
>* command.rs
>* main.rs
>* service.rs 封装call模块组件
>
> build.rs
>
> Cargo.toml
* pallets 具体的runtime模块，转账、治理等
> template 80行代码如果不用宏，展开将近900多行，缺点，debug在宏里。
>> src
>>* lib.rs 定义关联类型pub trait；声明decl_storage;decl_event;decl_error;decl_module;
>>* mock.rs
>>* test.rs
>>
>> Cargo.toml
* runtime 链上逻辑层
> src
>* lib.rs construct_runtime宏，控制用哪些，不用哪些pallet。对上面template pallets的封装及实例。
>
>build.rs
>
>Cargo.toml
* scripts

### 运行
>* cargo build --release
>* ./target/release/node-template purge-chain --dev
>* ./target/release/node-template --dev
>* 在polkadot.js.org查看本地链，链状态里有construct_runtime里包含的pallet。

### rust
* 学习trait对理解substrate特别有帮助

### 参与开源项目
* watch start
* issues
* pr pull request
* community
>* 官方聊天室 riot.im/app/#/room/!HzySYSalhtyWrwiwEV:matrix.org

### v3.0.0
* 代码库https://github.com/substrate-developer-hub/substrate-node-template/tree/v3.0.0+1

## 作业
### 第一题，请填写以下问题
（一下问题都可以在substrate官方文档里找到）
* Substrate的官方文档网址是什么？
> 回答：https://substrate.io; https://substrate.dev; https://substrate.dev/docs/en
* Substrate的recipe网址是什么？
> 回答：https://substrate.dev/recipes/
* Substrate的rust docs的网址是什么？
> 回答：https://substrate.dev/rustdocs/v3.0.0/sc_service/index.html
* Substrate的tutorial的网址是什么？
> 回答：https://substrate.dev/en/tutorials
* Substrate Seminar在每周几进行？
> 回答：Tuesday at 14:00UTC.
### 第二题：github的基本操作，养成良好的开源社区开发习惯
star和watch substrate和polkadot的repo，并截屏；
> 回答：![git watch start.png](git%20watch%20start.png)
### 第三题：请编译第一课中的node-template并截图编译成功的界面
* 代码库https://github.com/substrate-developer-hub/substrate-node-template/tree/v3.0.0+1
> 回答：![v3 build.png](v3%20build.png)
### 第四题：请运行node-template节点并截图
* 代码库https://github.com/substrate-developer-hub/substrate-node-template/tree/v3.0.0+1
> 回答：![v3 run dev.png](v3%20run%20dev.png)
