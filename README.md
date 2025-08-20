# Anchor Social

Anchor Social 是一个基于 Solana 区块链和 Anchor 框架开发的去中心化社交应用原型，支持用户创建个人资料、发布推文、点赞推文，并集成了代币奖励机制（点赞时会 mint 代币给用户）。

## 功能特点

- **用户资料管理**：创建和查询用户个人资料
- **推文功能**：发布推文并存储在区块链上
- **点赞互动**：对推文进行点赞，同时触发代币奖励
- **代币系统**：创建自定义代币 mint，并在点赞时 mint 代币给用户

## 技术栈

- **区块链**：Solana
- **开发框架**：Anchor
- **智能合约语言**：Rust
- **客户端语言**：TypeScript
- **代币标准**：SPL Token
- **元数据标准**：MPL Token Metadata

## 快速开始

### 前置依赖

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation)
- Node.js & Yarn
- Rust

### 安装步骤

1. 克隆仓库并进入项目目录
```bash
git clone <repository-url>
cd anchor_social
```

2. 安装依赖
```bash
yarn install
```

3. 构建项目
```bash
anchor build
```

4. 启动本地 Solana 验证器
```bash
anchor localnet start
```

5. 部署程序到本地网络
```bash
anchor deploy
```

## 使用示例

### 1. 创建用户资料
```typescript
import { createProfile } from "./app/api/profile";
import { useDefaultWallet } from "./app/api/wallet";

const wallet = useDefaultWallet();
await createProfile(wallet, "My Username");
```

### 2. 发布推文
```typescript
import { createTweet } from "./app/api/tweet";

const [tweetPda, txSignature] = await createTweet(wallet, "Hello, Solana!");
console.log("Tweet created at:", tweetPda.toString());
```

### 3. 点赞推文
```typescript
import { createLike } from "./app/api/tweet";

// 对指定推文进行点赞（会自动 mint 奖励代币）
await createLike(visitorWallet, tweetPda);
```

### 4. 创建代币 Mint
```typescript
import { createTokenMintAccount } from "./app/api/token";

const [tokenPda, txSignature] = await createTokenMintAccount(wallet);
console.log("Token mint created at:", tokenPda.toString());
```

## 项目结构

- `programs/anchor_social/src/`：Rust 智能合约代码
  - `instructions/`：交易指令处理逻辑（profile/tweet/token 相关操作）
  - `state/`：区块链存储的数据结构定义
  - `lib.rs`：程序入口点
  
- `app/`：TypeScript 客户端代码
  - `api/`：与智能合约交互的 API 封装
  - `index.ts`：示例使用代码

- `tests/`：测试文件
- `Anchor.toml`：Anchor 项目配置
- `Cargo.toml`：Rust 项目依赖配置

## 程序 ID

- 本地网络：`35vQtxXXv5rb99eiVrVVrwYMRYc7vscZvXas8zjEnnK5`

## 许可证

ISC 许可证（详见 LICENSE 文件）