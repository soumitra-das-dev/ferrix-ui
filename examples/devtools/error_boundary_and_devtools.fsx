// Example demonstrating ErrorBoundary and DevTools overlay integration
import { Column, Row, Text, Button } from "ferrix/ui";
import { useState } from "ferrix/reactivity";
import { render_with_error_boundary } from "ferrix/error";

export function DevToolsDemo(): Element {
  const showDevOverlay = useState(true);
  const triggerBug = useState(false);

  if (triggerBug.value) {
    throw new Error("Simulated runtime component crash!");
  }

  return (
    <Column padding={16} spacing={12}>
      <Text size={22}>Ferrix Error & DevTools Verification</Text>
      <Text>Overlay Active: {showDevOverlay.value ? "Yes" : "No"}</Text>

      <Row spacing={8}>
        <Button
          accessibilityLabel="Toggle DevTools Overlay"
          onClick={() => showDevOverlay.set(!showDevOverlay.value)}
        >
          Toggle Overlay
        </Button>
        <Button
          accessibilityLabel="Simulate Component Crash"
          onClick={() => triggerBug.set(true)}
        >
          Simulate Crash
        </Button>
      </Row>
    </Column>
  );
}
