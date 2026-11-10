import {
    Column,
    Row,
    Text,
    Button,
    Divider,
    Card,
    Badge,
    Switch
} from "ferrix/ui";

import {
    useWindowDimensions,
    useKeyboard,
    useAppState,
    useNetInfo,
    Platform,
    Vibration,
    Linking,
    Share,
    Alert,
    Clipboard
} from "ferrix/platform";

export function PlatformShowcase(): Element {
    const dims = useWindowDimensions();
    const keyboard = useKeyboard();
    const appState = useAppState();
    const net = useNetInfo();

    return (
        <Column padding={20} spacing={16}>
            <Text size={24} weight="bold">
                Ferrix Platform Showcase (Wave 3)
            </Text>

            <Divider />

            {/* Device & Screen Dimensions */}
            <Column spacing={6}>
                <Text size={18} weight="bold">Window Dimensions</Text>
                <Text>Width: {dims.value.width} dp</Text>
                <Text>Height: {dims.value.height} dp</Text>
                <Text>Scale: {dims.value.scale}x</Text>
                <Text>Font Scale: {dims.value.font_scale}x</Text>
            </Column>

            <Divider />

            {/* Platform Metadata */}
            <Column spacing={6}>
                <Text size={18} weight="bold">Platform Metadata</Text>
                <Text>OS: {Platform.OS}</Text>
                <Text>API Level: {Platform.version()}</Text>
            </Column>

            <Divider />

            {/* Keyboard & App State */}
            <Column spacing={6}>
                <Text size={18} weight="bold">Runtime Status</Text>
                <Text>Keyboard Visible: {keyboard.value.is_visible ? "Yes" : "No"}</Text>
                <Text>App State: {appState.value}</Text>
                <Text>Connected: {net.value.is_connected ? "Online" : "Offline"}</Text>
                <Text>Network Type: {net.value.is_wifi ? "Wi-Fi" : "Cellular"}</Text>
            </Column>

            <Divider />

            {/* Platform Actions */}
            <Column spacing={10}>
                <Text size={18} weight="bold">Native Actions</Text>

                <Button onClick={() => Vibration.vibrate(200)}>
                    Haptic Vibrate (200ms)
                </Button>

                <Button onClick={() => Share.share("Ferrix Framework", "Check out Ferrix: Reactive AOT Android UI!")}>
                    Share Ferrix
                </Button>

                <Button onClick={() => Alert.alert("Battery", "Battery optimization is active.")}>
                    Show Alert Dialog
                </Button>

                <Button onClick={() => Clipboard.setString("https://github.com/ferrix-ui")}>
                    Copy Link to Clipboard
                </Button>

                <Button onClick={() => Linking.openURL("https://github.com/ferrix-ui")}>
                    Open in Browser
                </Button>
            </Column>
        </Column>
    );
}

