// Example .fsx component showcasing clean formatting and linter compliance
import { Column, Text, Button } from "ferrix/ui";
import { useState } from "ferrix/reactivity";

export function ToolingDemo(): Element {
    // Hooks called at top level (satisfies rules_of_hooks)
    const count = useState(0);

    return (
        <Column padding={16} spacing={12}>
            <Text size={20}>Tooling Verification</Text>
        <Text>Current count: {count.value}</Text>
    <Button
        accessibilityLabel="Increment counter button"
        onClick={() => count.set(count.value + 1)}
        >
        Increment
    </Button>
</Column>
);
}
