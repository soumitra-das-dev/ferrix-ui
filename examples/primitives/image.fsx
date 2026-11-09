import { Column, Image, Text } from "ferrix/ui";

export function ImageDemo(): Element {
    return (
        <Column padding={20} spacing={12}>
            <Text size={20}>Image Primitive Demo</Text>
            <Image
                url="https://ferrix.dev/logo.png"
                contentDescription="Ferrix Logo"
            />
        </Column>
    );
}
