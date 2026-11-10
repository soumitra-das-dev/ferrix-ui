import { Column, Row, Text, Button, Switch, TextInput } from "ferrix/ui";
import { useState } from "ferrix/reactivity";

export function App(): Element {
  const enabled = useState(true);
  const text = useState("");

  return (
    <Column padding={20} spacing={12}>
      <Text size={24}>Ferrix Kitchen Sink</Text>
      <Row spacing={8}>
        <Text>Feature Enabled:</Text>
        <Switch value={enabled.value} onValueChange={(val) => enabled.set(val)} />
      </Row>
      <TextInput placeholder="Enter text here..." value={text.value} onChangeText={(t) => text.set(t)} />
      <Button onClick={() => enabled.set(!enabled.value)}>Toggle Feature</Button>
    </Column>
  );
}
