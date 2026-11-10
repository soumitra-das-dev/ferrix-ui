import { Column, Text, Button } from "ferrix/ui";
import { useState } from "ferrix/reactivity";

export function App(): Element {
  const count = useState(0);

  return (
    <Column padding={20} spacing={12}>
      <Text size={24}>Ferrix Counter</Text>
      <Text>Count: {count.value}</Text>
      <Button onClick={() => count.set(count.value + 1)}>
        Increment
      </Button>
    </Column>
  );
}
