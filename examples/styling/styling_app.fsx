import { Column, Text, Button, ScrollView } from "ferrix/ui";
import "./global.css";
import styles from "./card.module.css";

type StylingAppProps = {
    initialTheme: "light" | "dark";
};

export function StylingApp(props: StylingAppProps): Element {
    const isDark = useState(props.initialTheme === "dark");

    return (
        <Column className="container">
            <Text className="title">Ferrix Native Styling Showcase</Text>
            <Button
                onClick={() => isDark.set(!isDark.value)}
                style={{
                    backgroundColor: "#2563EB",
                    padding: 12,
                    borderRadius: 8,
                    marginBottom: 16
                }}
            >
                Toggle Theme
            </Button>
            <ScrollView>
                <Column className={styles.card}>
                    <Text className={styles.cardHeader}>CSS Module Card 1</Text>
                    <Text className={styles.cardBody}>
                        Card compiled at build time to native Style IR and interned style IDs.
                    </Text>
                </Column>
                <Column className={styles.card}>
                    <Text className={styles.cardHeader}>CSS Module Card 2</Text>
                    <Text className={styles.cardBody}>
                        Zero runtime CSS parsing. Instant native theme switching.
                    </Text>
                </Column>
            </ScrollView>
        </Column>
    );
}
