import { Column, Text, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function HomeScreen(): Element {
    const router = useRouter();

    return (
        <Column style={{ flex: 1, padding: 24, justifyContent: "center", alignItems: "center" }}>
            <Text style={{ fontSize: 24, fontWeight: "bold", color: "#f8fafc", marginBottom: 16 }}>
                Welcome to Ferrix Router
            </Text>
            <Button
                title="View Item 42"
                onPress={() => router.push("/details/42")}
            />
            <Button
                title="Sign In"
                onPress={() => router.push("/login")}
            />
        </Column>
    );
}
