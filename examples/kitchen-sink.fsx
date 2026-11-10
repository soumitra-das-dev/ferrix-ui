import { Column, Row, Text, Button, Image, TextInput, ScrollView, Switch, ActivityIndicator } from "ferrix/ui";

export function KitchenSink(): Element {
    const isToggled = useState(false);
    const counter = useState(0);

    return (
        <ScrollView padding={16} spacing={12}>
            <Text size={24}>Ferrix Primitive Kitchen Sink</Text>
            <Text size={16}>Demonstrating Wave 1 React Native Parity</Text>

            <Image
                url="https://ferrix.dev/banner.png"
                contentDescription="Banner Image"
            />

            <TextInput
                hint="Interactive EditText..."
                size={16}
                padding={12}
            />

            <Row spacing={8}>
                <Button
                    onClick={() => counter.set(counter.value + 1)}
                    backgroundColor={0xFF2563EB}
                >
                    Tap Count: {counter.value}
                </Button>
            </Row>

            <ActivityIndicator
                color={0xFF3B82F6}
            />
        </ScrollView>
    );
}

