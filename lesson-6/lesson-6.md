# Polkadot-JS

## 文档
* substrate.dev
* tutorials
* knowledge base
* Recipes
* Rustdocs
* polkadot wiki
* polkadot js

## polkadot js app


## 课后作业
* 1. 访问Polkadot-JS App的网址是什么？
>* A. https://poldot.com
>* B. https://pokadot.js.com/apps
>* C. https://pokadot.js.org/apps
>* D. https://pokadot.js/org/apps

答案：C

* 2. Polkadot-JS App 和 Polkadot-Js API的区别是什么？(多选)
>* A. 名字上一看就有不同，一个尾缀是App，一个尾缀是API，你瞎了吗？
>* B. Polkadot-JS App提供了官方的JS库去连Substrate网络，而Polkadot-JS API则是官方的前端与
>Substrate网络交互。
>* C. Polkadot-JS App是官方的前端与Substrate网络交互，而Polkadot-JS API则提供了官方的JS库去连
>Substrate网络。
>* D. Polkadot-JS App是官方的Polkadot钱包，而Polkadot-JS API则提供了官方的JS库去连Substrate网络。
>* E. Polkadot-JS App是官方的Polkadot钱包，而Polkadot-JS API则提供了官方的JS库去连现在主流的区块链
>网络（Substrate，以太坊，比特币等）。

答案：A、C、D

* 3. 你可以在Polkadot-JS App内做什么操作？（多选）
>* A. 查看Substrate网络区块信息
>* B. 对Substrate网络作出交易（Extrinsics）
>* C. 有一个javascript编辑器，可对Substrate网络写出基础javascript与之互动
>* D. 是Substrate的水龙头（faucet）可取得小额Substrate的代币
>* E. 可以对一个信息以某个账户作签名。

答案： A、B、C、E

* 4. 以下哪些生产环境的网络（LIVE NETWORK）是Polkadot-JS App里默认有支持的？（多选）
>* A. Kusama
>* B. Acala
>* C. Kulupu
>* D. Centrifuge
>* E. ChainX

答案：A、C、D、E

* 5. 如果在Substrate端加了自定义类型，我们在Polkadot-JS App里
需要作什么才能支持连到这个Substrate节点？（多选）
>* A. 在Setting里，Metadata tab里，加自定义的JSON对象。
>* B. 在Setting里，Developer tab里，加自定义的XML对象。
>* C. 在Setting里，Developer tab里，加自定义的JSON对象。
>* D. 在Toolbox里，Sign message tab里，先发一个签了名的信息作核实。
>* E. 在Toolbox里，Verify signature tab里，先发一个签了名的信息作核实。

答案：C

* 6. 在Polkadot-JS API里，数字默认是用哪个类型代表？
>* A. JS里默认的数字类型
>* B. 字符串
>* C. [bn.js](https://github.com/indutny/bn.js/)
>* D. [bignumber.js](https://github.com/MikeMcl/bignumber.js/)
>* E. [decimal.js](https://github.com/MikeMcl/decimal.js/)

答案：C

* 7. 我要查询Substrate链上的存储变量，并订阅它的变更，应该用以下哪个方法？
>* A. `const val = await api.query..()`
>* B. `const unsub = await api.query..(value => {...})`
>* C. `const val = await api.consts..()`
>* D. `const val = await api.tx..().signAndSend()`
>* E. `const val = await api.tx..().signAndSend(value => {...})`

答案： B

* 8. 我要对Substrate链上发出一次交易，但不需要订阅交易处理状态，应该用以下哪个方法？
>* A. `const val = await api.query..()`
>* B. `const unsub = await api.query..(value => {...})`
>* C. `const val =await api.consts..()`
>* D. `const val = await api.tx..().signAndSend()`
>* E. `const val = await api.tx..().signAndSend(value => {...})`

答案：D

* 9. Polkadot-JS API内哪个组件负责取得[Polkadot-JS extension](https://github.com/polkadot-js/extension))
里的钱包资料？

答案：api.rpc.state.getMatadata()

* 10. 现在在Github上的[Substrate repo](https://github.com/paritytech/substrate)约有多少用户给它打了星？选个最接近的。
>* A. 500
>* B. 1000
>* C. 3000
>* D. 5000
>* E. 7500

答案：D
