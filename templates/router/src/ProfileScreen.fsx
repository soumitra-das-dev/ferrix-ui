import { Column, Row, Text, Button, Switch } from "ferrix/ui";
import { useState } from "ferrix/reactivity";
import { useRouter } from "ferrix/router";

export function ProfileScreen(): Element {
  const pushNotifications = useState(true);
  const router = useRouter();

  return (
    <Column padding={20} spacing={16}>
      <Text size={26}>Profile Settings</Text>
      <Row spacing={8}>
        <Text>Push Notifications:</Text>
        <Switch
          value={pushNotifications.value}
          onValueChange={(val) => pushNotifications.set(val)}
        />
      </Row>
      <Button onClick={() => router.back()}>Back to Home</Button>
    </Column>
  );
}
