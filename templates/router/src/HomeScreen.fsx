import { Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function HomeScreen(): Element {
  const router = useRouter();

  return (
    <Column padding={20} spacing={16}>
      <Text size={26}>Home Screen</Text>
      <Text>Welcome to the multi-file Ferrix Router application.</Text>
      <Button onClick={() => router.push("/details")}>View Details</Button>
      <Button onClick={() => router.push("/profile")}>View Profile</Button>
    </Column>
  );
}
