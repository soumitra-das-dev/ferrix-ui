import { Row, Text } from "ferrix/ui";

export function Header(): Element {
  return (
    <Row padding={16} spacing={8}>
      <Text size={20}>Ferrix App</Text>
    </Row>
  );
}
