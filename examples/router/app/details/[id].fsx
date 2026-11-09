import { Column, Text, Button } from "ferrix/ui";
import { useRouter, useLocalSearchParams } from "ferrix/router";

export function DetailsScreen(): Element {
    const router = useRouter();
    const params = useLocalSearchParams();

    return (
        <Column style={{ flex: 1, padding: 24, justifyContent: "center", alignItems: "center" }}>
            <Text style={{ fontSize: 20, color: "#f8fafc", marginBottom: 12 }}>
                Item Details
            </Text>
            <Text style={{ fontSize: 16, color: "#94a3b8", marginBottom: 20 }}>
                ID: {params.id}
            </Text>
            <Button
                title="Go Back"
                onPress={() => router.back()}
            />
        </Column>
    );
}
