import { Column, Text } from "ferrix/ui";

type TextExampleProps = {
    headline: String,
    content: String,
};

export function TextExample(props: TextExampleProps): Element {
    return (
        <Column padding={20} spacing={10}>
            <Text size={24} color={0xFF111827} numberOfLines={1}>
                {props.headline}
            </Text>
            <Text size={16} color={0xFF374151} numberOfLines={3}>
                {props.content}
            </Text>
        </Column>
    );
}
