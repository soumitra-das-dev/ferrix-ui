import { Column, TextInput, Text } from "ferrix/ui";

export function TextInputDemo(): Element {
    const textState = useState("Initial Query");

    return (
        <Column padding={20} spacing={12}>
            <Text size={20}>Controlled TextInput Demo</Text>
            <TextInput
                hint="Type your thoughts..."
                size={16}
                padding={12}
            />
            <Text size={14}>Current: {textState.value}</Text>
        </Column>
    );
}

