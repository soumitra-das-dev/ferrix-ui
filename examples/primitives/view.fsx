import { View, Text } from "ferrix/ui";

type ViewExampleProps = {
    title: String,
};

export function ViewExample(props: ViewExampleProps): Element {
    return (
        <View padding={16} backgroundColor={0xFFF3F4F6}>
            <View padding={12} backgroundColor={0xFFFFFFFF} borderRadius={8} elevation={2}>
                <Text size={20} color={0xFF1F2937}>{props.title}</Text>
                <Text size={14} color={0xFF4B5563}>Styled box container with elevation and padding.</Text>
            </View>
        </View>
    );
}

