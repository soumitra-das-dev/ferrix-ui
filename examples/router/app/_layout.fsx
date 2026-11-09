import { Column, View, Text } from "ferrix/ui";

export function RootLayout(props: { children: Element }): Element {
    return (
        <Column style={{ flex: 1, backgroundColor: "#0f172a" }}>
            <View style={{ height: 56, backgroundColor: "#1e293b", justifyContent: "center", paddingHorizontal: 16 }}>
                <Text style={{ color: "#38bdf8", fontSize: 18, fontWeight: "bold" }}>Ferrix App</Text>
            </View>
            <View style={{ flex: 1 }}>
                {props.children}
            </View>
        </Column>
    );
}
