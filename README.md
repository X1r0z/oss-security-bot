# OSS Security Bot

[oss-security](https://www.openwall.com/lists/oss-security/) 飞书推送机器人, 基于 Rust 编写 🦀

使用 LLM 对邮件内容进行总结 (Summarize)

## 配置

程序初次运行时会生成一份 Config.toml

```toml
[oss]
interval = 60

[bot]
access_token = "<LARK_ACCESS_TOKEN>" # starts with https://open.feishu.cn/open-apis/bot/v2/hook/
secret_key = "<LARK_SECRET_KEY>"

[llm]
base_url = "<BASE_URL>"
api_key = "<API_KEY>"
model = "<MODEL>"

system = "你是一名经验丰富的网络安全研究员 (Security Researcher)"
user = '''请结合以下要求总结文本:
1. 使用中文输出总结后的内容
2. 仅总结邮件正文部分, 忽略邮件的 Metadata 信息
3. 仅需输出总结后的内容

待总结的文本如下:
{TEXT}'''
```

按照要求配置飞书 Webhook 机器人以及 LLM API (兼容 OpenAI 规范)

![](assets/lark-bot-configuration.png)

## 使用

![](assets/security-advisory-1.png)

![](assets/security-advisory-2.png)