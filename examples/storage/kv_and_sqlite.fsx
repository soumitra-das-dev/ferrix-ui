import { Column, View, Text, TextInput, Button } from "ferrix/ui";
import { useStorage } from "ferrix/storage";
import { useLiveQuery } from "ferrix/sqlite";

type Task = {
    id: i64,
    title: string,
    completed: boolean,
};

export function StorageAndSqliteDemo(): Element {
    // 1. Reactive KV Storage
    const [theme, setTheme] = useStorage("app_theme", "dark");

    // 2. Reactive SQLite Live Query
    const tasks = useLiveQuery<Task>("tasks", "SELECT id, title, completed FROM tasks ORDER BY id DESC");

    return (
        <Column style={{ flex: 1, backgroundColor: theme === "dark" ? "#0f172a" : "#f8fafc", padding: 16 }}>
            <View style={{ marginBottom: 24, padding: 16, backgroundColor: theme === "dark" ? "#1e293b" : "#ffffff", borderRadius: 8 }}>
                <Text style={{ fontSize: 18, fontWeight: "bold", color: theme === "dark" ? "#f8fafc" : "#0f172a", marginBottom: 8 }}>
                    KV Store Preference
                </Text>
                <Text style={{ fontSize: 14, color: "#94a3b8", marginBottom: 12 }}>
                    Current theme: {theme}
                </Text>
                <Button
                    title={`Switch to ${theme === "dark" ? "Light" : "Dark"} Mode`}
                    onPress={() => setTheme(theme === "dark" ? "light" : "dark")}
                />
            </View>

            <View style={{ flex: 1, padding: 16, backgroundColor: theme === "dark" ? "#1e293b" : "#ffffff", borderRadius: 8 }}>
                <Text style={{ fontSize: 18, fontWeight: "bold", color: theme === "dark" ? "#f8fafc" : "#0f172a", marginBottom: 12 }}>
                    SQLite Live Tasks ({tasks.length})
                </Text>
                {tasks.map((task) => (
                    <View key={task.id.toString()} style={{ paddingVertical: 8, borderBottomWidth: 1, borderBottomColor: "#334155" }}>
                        <Text style={{ fontSize: 16, color: task.completed ? "#64748b" : (theme === "dark" ? "#ffffff" : "#000000") }}>
                            {task.completed ? "✓ " : "○ "} {task.title}
                        </Text>
                    </View>
                ))}
            </View>
        </Column>
    );
}

