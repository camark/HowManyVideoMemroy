# 需要多少显存？

一款 Tauri v2 桌面应用程序，用于计算 AI/LLM 模型在不同量化级别下的显存需求。

## 功能特性

- **快速显存估算** — 获取流行 LLM 模型的显存需求
- **多量化级别** — 支持 FP16、FP32、INT8、INT4 计算
- **KV 缓存支持** — 可选包含上下文内存开销
- **深色模式** — 原生系统主题支持

## 安装

### 前置要求

- Node.js（v18 或更高版本）
- Rust（最新稳定版）
- Cargo Tauri CLI

### 设置

```bash
# 安装依赖
npm install

# 构建 CSS
npm run build:css

# 开发模式运行
cargo tauri dev

# 生产环境构建
cargo tauri build
```

## 使用方法

1. 输入模型名称（例如："Llama-3-8B"、"Mistral-7B"、"GPT-2"）
2. 选择量化级别（FP16、FP32、INT8、INT4）
3. 可选开启 KV 缓存内存
4. 点击"计算"查看显存需求

## 工作原理

显存计算流程：
1. **数据库查找** — 查询内部模型数据库获取已知模型
2. **参数提取** — 从模型名称解析参数量（例如："8b" → 80 亿）
3. **基础计算** — FP16 = 参数量 (亿) × 2.0 GB
4. **量化乘数**：
   - FP32 = FP16 × 2.0
   - FP16 = 基础值
   - INT8 = FP16 × 0.5
   - INT4 = FP16 × 0.25

## 技术栈

- **前端：** 原生 HTML/CSS/JavaScript + Tailwind CSS v4
- **后端：** Rust（Tauri v2 命令）
- **样式：** PostCSS + `@tailwindcss/postcss`

## 项目结构

```
HowManyVideoMemory/
├── src/                    # 前端源文件
│   ├── index.html          # 主界面
│   ├── main.js             # Tauri 命令调用
│   ├── styles.css          # Tailwind 导入 + 自定义样式
│   └── dist.css            # 生成的 CSS
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   └── lib.rs          # 显存计算逻辑
│   ├── Cargo.toml          # Rust 依赖
│   └── tauri.conf.json     # Tauri 配置
├── CLAUDE.md               # 项目文档
└── package.json            # Node 依赖
```

## 许可证

MIT
