import { Column, View, Text, Button } from "ferrix/ui";
import { useState } from "ferrix/reactivity";

export function StackAndTabsDemo(): Element {
    const activeTab = useState(0);

    return (
        <Column style={{ flex: 1, backgroundColor: "#0f172a" }}>
            <View style={{ height: 56, backgroundColor: "#1e293b", paddingHorizontal: 16, justifyContent: "center" }}>
                <Text style={{ fontSize: 18, color: "#ffffff", fontWeight: "bold" }}>
                    {activeTab.value === 0 ? "Feed" : activeTab.value === 1 ? "Explore" : "Profile"}
                </Text>
            </View>

            <View style={{ flex: 1, padding: 24, justifyContent: "center", alignItems: "center" }}>
                {activeTab.value === 0 && (
                    <Text style={{ fontSize: 20, color: "#38bdf8" }}>Home Feed Screen</Text>
                )}
                {activeTab.value === 1 && (
                    <Text style={{ fontSize: 20, color: "#4ade80" }}>Explore & Search Screen</Text>
                )}
                {activeTab.value === 2 && (
                    <Text style={{ fontSize: 20, color: "#f43f5e" }}>User Profile Screen</Text>
                )}
            </View>

            <View style={{ height: 60, flexDirection: "row", backgroundColor: "#1e293b", borderTopWidth: 1, borderTopColor: "#334155" }}>
                <Button title="Feed" onPress={() => activeTab.set(0)} style={{ flex: 1 }} />
                <Button title="Explore" onPress={() => activeTab.set(1)} style={{ flex: 1 }} />
                <Button title="Profile" onPress={() => activeTab.set(2)} style={{ flex: 1 }} />
            </View>
        </Column>
    );
}

