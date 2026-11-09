import { TabNavigator, Screen, Column, Text } from "ferrix/ui";

export function App(): Element {
  return (
    <TabNavigator>
      <Screen name="Home">
        <Column padding={20}><Text size={20}>Home Screen</Text></Column>
      </Screen>
      <Screen name="Explore">
        <Column padding={20}><Text size={20}>Explore Screen</Text></Column>
      </Screen>
      <Screen name="Profile">
        <Column padding={20}><Text size={20}>Profile Screen</Text></Column>
      </Screen>
    </TabNavigator>
  );
}
