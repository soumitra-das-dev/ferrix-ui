import { StackNavigator, Screen, Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function App(): Element {
  const router = useRouter();

  return (
    <StackNavigator>
      <Screen name="Home">
        <Column padding={20} spacing={16}>
          <Text size={26}>Ferrix App Router</Text>
          <Text>Multi-file architecture with separate screen components.</Text>
          <Button onClick={() => router.push("/details")}>Go to Details</Button>
          <Button onClick={() => router.push("/profile")}>Go to Profile</Button>
        </Column>
      </Screen>
      <Screen name="Details">
        <Column padding={20} spacing={16}>
          <Text size={26}>Details Screen</Text>
          <Text>Compiled ahead-of-time from src/DetailsScreen.fsx.</Text>
          <Button onClick={() => router.back()}>Back to Home</Button>
        </Column>
      </Screen>
      <Screen name="Profile">
        <Column padding={20} spacing={16}>
          <Text size={26}>User Profile</Text>
          <Text>Compiled ahead-of-time from src/ProfileScreen.fsx.</Text>
          <Button onClick={() => router.back()}>Back to Home</Button>
        </Column>
      </Screen>
    </StackNavigator>
  );
}
