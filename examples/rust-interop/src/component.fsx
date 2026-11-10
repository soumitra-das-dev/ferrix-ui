import { fib, Counter, Status } from "./math.rs";
import { Uuid } from "rust:uuid";
import { Regex } from "rust:regex";
import { json, Value } from "rust:serde_json";
import { Column, Text, Button } from "ferrix/ui";
import { useState } from "ferrix/reactivity";

rust {
    pub fn format_greeting(target: &str) -> String {
        format!("Hello, {}! (from inline Rust)", target)
    }

    pub fn generate_token() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    pub fn parse_metadata_key(json_str: &str, key: &str) -> String {
        let v: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::Value::Null);
        v.get(key)
            .and_then(|val| val.as_str())
            .unwrap_or("unknown")
            .to_string()
    }
}

export function App(): Element {
    const count = useState(5);
    const sessionToken = useState("init-token");
    const jsonSample = useState("{\"role\":\"admin\",\"org\":\"ferrix\"}");

    return (
        <Column padding={20} spacing={12}>
            <Text size={24}>Rust Interop Demo</Text>
            <Text>Fibonacci(5): {fib(5)}</Text>
            <Text>Fibonacci({count.value}): {fib(count.value)}</Text>
            <Text>Session Token: {sessionToken.value}</Text>
            <Text>Org: {parse_metadata_key(jsonSample.value, "org")}</Text>
            <Text>Greeting: {format_greeting("Developer")}</Text>
            <Button
                testID="btn-refresh"
                onClick={() => count.set(count.value + 1)}
            >
                Increment
            </Button>
            <Button
                testID="btn-token"
                onClick={() => sessionToken.set(generate_token())}
            >
                Generate Token
            </Button>
        </Column>
    );
}

// hot-reload-scratch-test
