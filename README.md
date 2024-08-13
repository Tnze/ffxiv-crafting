# ffxiv-crafting

A crafting simulator of FFXIV.

https://crates.io/crates/ffxiv-crafting  
https://ngabbs.com/read.php?&tid=29796644

---

### 本Crate实现的功能有

- 可以进行生产模拟(废话
- 拥有完整的技能支持及超高的模拟精度
- 可以检测并分类各种不能释放技能的情况
- 支持所有制作状态的模拟(什么白球红球蓝球彩球之类的)
- serde序列化及反序列化支持(需要开启feature: serde-support)
- 拥有完整有效的单元测试以及上传前随手写的详细中文API注释

### 可以用于

- 各位大佬制作自己的生产模拟器
- 进行计算生产宏算法方面的研究

### 使用方法包括但不限于

- 直接在Rust项目代码中使用
- 编译为dll并导出C接口以供其他语言调用
- 编译为Wasm并创建Js绑定以在Web项目中使用

### 适配版本

7.X
