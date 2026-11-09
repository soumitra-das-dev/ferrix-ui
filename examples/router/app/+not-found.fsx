import { Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function NotFoundScreen(): Element {
    const router = useRouter();

    return (
        <Column style={{ flex: 1, padding: 24, justifyContent: "center", alignItems: "center" }}>
            <Text style={{ fontSize: 28, fontWeight: "bold", color: "#ef4444", marginBottom: 12 }}>
                404 - Not Found
            </Text>
            <Text style={{ fontSize: 16, color: "#94a3b8", marginBottom: 24 }}>
                This screen does not exist.
            </Text>
            <Button
                title="Return Home"
                onPress={() => router.replace("/")}
            />
        </Column>
    );
}
