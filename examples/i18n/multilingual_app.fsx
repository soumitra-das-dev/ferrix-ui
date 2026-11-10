import { Column, View, Text, Button } from "ferrix/ui";
import { useTranslation } from "ferrix/i18n";

export function MultilingualApp(): Element {
    const { t, locale, setLocale } = useTranslation();

    return (
        <Column style={{ flex: 1, padding: 24, justifyContent: "center", backgroundColor: "#0f172a" }}>
            <View style={{ marginBottom: 24, padding: 16, backgroundColor: "#1e293b", borderRadius: 8 }}>
                <Text style={{ fontSize: 24, fontWeight: "bold", color: "#f8fafc", marginBottom: 12 }}>
                    {t("welcome_title", { name: "Developer" })}
                </Text>
                <Text style={{ fontSize: 16, color: "#94a3b8", marginBottom: 16 }}>
                    {t("unread_notifications", { count: 3 })}
                </Text>
                <Text style={{ fontSize: 14, color: "#38bdf8" }}>
                    Active Language: {locale.toUpperCase()}
                </Text>
            </View>

            <View style={{ flexDirection: "row", justifyContent: "space-between" }}>
                <Button title="English" onPress={() => setLocale("en")} />
                <Button title="Español" onPress={() => setLocale("es")} />
                <Button title="Français" onPress={() => setLocale("fr")} />
                <Button title="العربية (RTL)" onPress={() => setLocale("ar")} />
            </View>
        </Column>
    );
}
