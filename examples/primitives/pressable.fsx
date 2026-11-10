import { Column, Pressable, Text } from "ferrix/ui";

type PressableExampleProps = {
    label: String,
};

export function PressableExample(props: PressableExampleProps): Element {
    const pressedCount = useState(0);

    return (
        <Column padding={24} spacing={16}>
            <Text size={20}>Pressable Interactive Demo</Text>
            <Text size={16}>Pressed: {pressedCount.value}</Text>
            <Pressable
                padding={16}
                backgroundColor={0xFF2563EB}
                borderRadius={8}
                onClick={() => pressedCount.set(pressedCount.value + 1)}
            >
                <Text size={16} color={0xFFFFFFFF}>{props.label}</Text>
            </Pressable>
        </Column>
    );
}

