import { Column, Text, Button, ActivityIndicator } from "ferrix/ui";
import { useQuery } from "ferrix/network";

export function App(): Element {
  const query = useQuery("/api/data");

  return (
    <Column padding={20} spacing={12}>
      <Text size={22}>API Client Demo</Text>
      <ActivityIndicator color={0xFF3B82F6} />
      <Text>Live Network Response</Text>
      <Button onClick={() => query.refetch()}>Reload</Button>
    </Column>
  );
}
