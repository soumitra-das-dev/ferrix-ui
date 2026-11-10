import { Column, Text, Button } from "ferrix/ui";

export function App(): Element {
  return (
    <Column padding={24} spacing={16}>
      <Text size={24}>Welcome to Ferrix</Text>
      <Text>Your new native Android app is running on Rust.</Text>
      <Button onClick={() => null}>Get Started</Button>
    </Column>
  );
}
