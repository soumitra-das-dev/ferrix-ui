import { Column, Row, Stack, Text, Checkbox, Slider, ProgressBar, Badge, Icon, Divider, Center } from "ferrix/ui";

export function ExtendedKitchenSink(): Element {
    const isChecked = useState(true);
    const sliderVal = useState(50);

    return (
        <Column padding={16} spacing={12}>
            <Text size={24}>Wave 2 Extended UI Showcase</Text>
            <Divider color={0xFFE5E7EB} thickness={1} />

            <Row spacing={8}>
                <Checkbox
                    checked={isChecked.value}
                    onClick={() => isChecked.set(!isChecked.value)}
                />
                <Text size={16}>Enable Hardware Acceleration</Text>
            </Row>

            <Slider
                value={sliderVal.value}
            />

            <ProgressBar
                progress={75}
                color={0xFF10B981}
            />

            <Row spacing={8}>
                <Badge content="UPDATED" backgroundColor={0xFF3B82F6} />
                <Badge content="BETA" backgroundColor={0xFFEF4444} />
            </Row>

            <Center>
                <Icon name="check-circle" size={32} color={0xFF10B981} />
            </Center>
        </Column>
    );
}
