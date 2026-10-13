//! Model Context Protocol (MCP) JSON-RPC 2.0 Stdio Server for Aether.
//!
//! Exposes Aether's Gemini 3.8 Flash assistant, Semantic Vector Memory Vault,
//! Zero-Guilt sanitizer, and Harmonic Audio metadata to any MCP-compatible host.

use aether_core::{
    AetherDesignTokens, AetherPromptRequest, FocusSanctuary, GeminiClient, MemoryVault,
    ZeroGuiltFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: &'static str,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub struct McpServer {
    pub gemini: GeminiClient,
    pub vault: MemoryVault,
    pub sanctuary: FocusSanctuary,
}

impl Default for McpServer {
    fn default() -> Self {
        Self {
            gemini: GeminiClient::default(),
            vault: MemoryVault::with_sanctuary_seeds(),
            sanctuary: FocusSanctuary::default(),
        }
    }
}

impl McpServer {
    pub async fn handle_request(&mut self, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
        let id = req.id.clone().unwrap_or(Value::Null);

        // Notifications without an id do not expect a response
        if req.method.starts_with("notifications/") {
            return None;
        }

        let result = match req.method.as_str() {
            "initialize" => Ok(json!({
                "protocolVersion": "2025-06-18",
                "serverInfo": {
                    "name": "aether-mcp-server",
                    "version": env!("CARGO_PKG_VERSION"),
                    "primaryAiModel": "gemini-3.8-flash",
                    "fallbackAiModel": "gemini-2.5-flash"
                },
                "capabilities": {
                    "tools": {}
                }
            })),
            "tools/list" => Ok(json!({
                "tools": [
                    {
                        "name": "aether_ask_gemini",
                        "description": "Queries Aether's Gemini 3.8 Flash assistant (with Gemini 2.5 Flash & Zero-Guilt local fallback).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "prompt": { "type": "string" },
                                "harmonic_hz": { "type": "number" }
                            },
                            "required": ["prompt"]
                        }
                    },
                    {
                        "name": "aether_memory_search",
                        "description": "Searches Aether's local semantic vector MemoryVault using cosine similarity.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "query": { "type": "string" },
                                "top_k": { "type": "integer" }
                            },
                            "required": ["query"]
                        }
                    },
                    {
                        "name": "aether_memory_add",
                        "description": "Adds a Zero-Guilt verified note into Aether's semantic vector MemoryVault.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "content": { "type": "string" },
                                "tags": { "type": "array", "items": { "type": "string" } }
                            },
                            "required": ["content"]
                        }
                    },
                    {
                        "name": "aether_zero_guilt_sanitize",
                        "description": "Transforms text to comply with Aether's Zero-Guilt Philosophy (Sıfır Suçluluk).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "text": { "type": "string" }
                            },
                            "required": ["text"]
                        }
                    },
                    {
                        "name": "aether_design_tokens",
                        "description": "Returns Aether's official Dark Frosted Glassmorphism DESIGN.md tokens.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    }
                ]
            })),
            "tools/call" => self.call_tool(&req.params).await,
            _ => Err(json!({
                "code": -32601,
                "message": format!("Method not found: {}", req.method)
            })),
        };

        match result {
            Ok(val) => Some(JsonRpcResponse {
                jsonrpc: "2.0",
                id,
                result: Some(val),
                error: None,
            }),
            Err(err) => Some(JsonRpcResponse {
                jsonrpc: "2.0",
                id,
                result: None,
                error: Some(err),
            }),
        }
    }

    async fn call_tool(&mut self, params: &Value) -> Result<Value, Value> {
        let name = params["name"].as_str().unwrap_or_default();
        let args = &params["arguments"];

        match name {
            "aether_ask_gemini" => {
                let prompt = args["prompt"].as_str().unwrap_or("Merhaba Aether").to_string();
                let harmonic_hz = args["harmonic_hz"].as_f64().map(|v| v as f32);
                let context_notes: Vec<String> = self
                    .vault
                    .search(&prompt, 2)
                    .into_iter()
                    .map(|m| m.entry.content)
                    .collect();

                let resp = self
                    .gemini
                    .generate(&AetherPromptRequest {
                        prompt,
                        context_notes,
                        harmonic_hz,
                    })
                    .await;

                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&resp).unwrap_or_default()
                    }]
                }))
            }
            "aether_memory_search" => {
                let query = args["query"].as_str().unwrap_or_default();
                let top_k = args["top_k"].as_u64().unwrap_or(3) as usize;
                let hits = self.vault.search(query, top_k);
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&hits).unwrap_or_default()
                    }]
                }))
            }
            "aether_memory_add" => {
                let content = args["content"].as_str().unwrap_or_default();
                let tags: Vec<String> = args["tags"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let entry = self.vault.add(content, tags);
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&entry).unwrap_or_default()
                    }]
                }))
            }
            "aether_zero_guilt_sanitize" => {
                let text = args["text"].as_str().unwrap_or_default();
                let report = ZeroGuiltFilter::sanitize(text);
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&report).unwrap_or_default()
                    }]
                }))
            }
            "aether_design_tokens" => {
                let tokens = AetherDesignTokens::default();
                Ok(json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&tokens).unwrap_or_default()
                    }]
                }))
            }
            _ => Err(json!({
                "code": -32602,
                "message": format!("Unknown tool: {}", name)
            })),
        }
    }

    /// Runs the JSON-RPC 2.0 MCP loop over standard input/output.
    pub async fn run_stdio(&mut self) -> anyhow::Result<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin).lines();

        while let Some(line) = reader.next_line().await? {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(trimmed)
                && let Some(resp) = self.handle_request(req).await
            {
                let out = serde_json::to_string(&resp)?;
                stdout.write_all(out.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_initialize_and_tool_call() {
        let mut server = McpServer::default();
        let init_req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(1)),
            method: "initialize".into(),
            params: json!({}),
        };
        let init_resp = server.handle_request(init_req).await.unwrap();
        assert_eq!(
            init_resp.result.unwrap()["serverInfo"]["primaryAiModel"],
            "gemini-3.8-flash"
        );

        let tool_req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(json!(2)),
            method: "tools/call".into(),
            params: json!({
                "name": "aether_zero_guilt_sanitize",
                "arguments": {
                    "text": "Bugün su içmeyi ihmal ettin ve geç kaldın"
                }
            }),
        };
        let tool_resp = server.handle_request(tool_req).await.unwrap();
        let text = tool_resp.result.unwrap()["content"][0]["text"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(text.contains("dinlenme molası"));
        assert!(text.contains("tam zamanında buradasın"));
    }
}
