import { Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function DetailsScreen(): Element {
  const router = useRouter();

  return (
    <Column padding={20} spacing={16}>
      <Text size={26}>Details Screen</Text>
      <Text>Compiled from a separate modular component in src/DetailsScreen.fsx.</Text>
      <Button onClick={() => router.back()}>Back to Home</Button>
    </Column>
  );
}
