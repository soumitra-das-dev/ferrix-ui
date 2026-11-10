import { Column, Text, Button, ActivityIndicator } from "ferrix/ui";
import { useQuery } from "ferrix/network";

export function App(): Element {
  const query = useQuery("/api/data");

  return (
    <Column padding={20} spacing={12}>
      <Text size={22}>API Client Demo</Text>
      {query.loading && <ActivityIndicator />}
      {query.data && <Text>Result: {query.data}</Text>}
      <Button onClick={() => query.refetch()}>Reload</Button>
    </Column>
  );
}
