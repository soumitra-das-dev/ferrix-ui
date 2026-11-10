import { StackNavigator, Screen, Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function App(): Element {
  const router = useRouter();
  return (
    <StackNavigator>
      <Screen name="Overview">
        <Column padding={20} spacing={12}>
          <Text size={22}>Master Overview</Text>
          <Button onClick={() => router.push("/details")}>View Details</Button>
        </Column>
      </Screen>
      <Screen name="Details">
        <Column padding={20} spacing={12}>
          <Text size={22}>Detail View</Text>
          <Button onClick={() => router.back()}>Go Back</Button>
        </Column>
      </Screen>
    </StackNavigator>
  );
}
