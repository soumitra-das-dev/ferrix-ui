import { Column, Text, TextInput, Button } from "ferrix/ui";
import { useRouter } from "ferrix/router";

export function LoginScreen(): Element {
    const router = useRouter();

    return (
        <Column style={{ flex: 1, padding: 24, justifyContent: "center" }}>
            <Text style={{ fontSize: 24, color: "#f8fafc", marginBottom: 20 }}>
                Sign In
            </Text>
            <TextInput
                placeholder="Username or email"
                style={{ backgroundColor: "#1e293b", padding: 12, borderRadius: 8, color: "#ffffff", marginBottom: 16 }}
            />
            <Button
                title="Authenticate"
                onPress={() => router.replace("/")}
            />
        </Column>
    );
}

