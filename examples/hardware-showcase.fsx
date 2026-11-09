import {
    Column,
    Row,
    Text,
    Button,
    Divider,
    Card,
    Badge,
    Switch,
    Slider
} from "ferrix/ui";

import {
    Camera,
    Sensors,
    Audio,
    Haptics,
    SecureStore,
    FileSystem,
    Notifications,
    LocalAuthentication
} from "ferrix/modules";

export function HardwareShowcase(): Element {
    const isEnrolled = useState(true);
    const accelData = useState("X: 0.0, Y: 9.8, Z: 0.1");
    const secretValue = useState("Saved: None");

    return (
        <Column padding={20} spacing={14}>
            <Text size={24} weight="bold">
                Ferrix Hardware Showcase (Wave 4)
            </Text>

            <Divider />

            {/* Sensors & Haptics */}
            <Column spacing={8}>
                <Text size={18} weight="bold">Motion & Haptics</Text>
                <Text>Accelerometer: {accelData.value}</Text>

                <Row spacing={10}>
                    <Button onClick={() => Haptics.impact("light")}>
                        Light Haptic
                    </Button>
                    <Button onClick={() => Haptics.impact("heavy")}>
                        Heavy Haptic
                    </Button>
                </Row>
            </Column>

            <Divider />

            {/* Secure Store */}
            <Column spacing={8}>
                <Text size={18} weight="bold">Hardware Keystore</Text>
                <Text>{secretValue.value}</Text>

                <Row spacing={10}>
                    <Button onClick={() => {
                        SecureStore.setItem("token", "super_secret_jwt_xyz");
                        secretValue.set("Saved: super_secret_jwt_xyz");
                    }}>
                        Store Secret
                    </Button>

                    <Button onClick={() => {
                        const val = SecureStore.getItem("token");
                        secretValue.set("Retrieved: " + val);
                    }}>
                        Read Secret
                    </Button>
                </Row>
            </Column>

            <Divider />

            {/* Biometrics */}
            <Column spacing={8}>
                <Text size={18} weight="bold">Biometric Authentication</Text>
                <Text>Biometrics Enrolled: {isEnrolled.value ? "Yes" : "No"}</Text>

                <Button onClick={() => {
                    LocalAuthentication.authenticate("Unlock Ferrix Secure Store");
                }}>
                    Prompt Biometrics
                </Button>
            </Column>

            <Divider />

            {/* Media & Audio */}
            <Column spacing={8}>
                <Text size={18} weight="bold">Audio & Media</Text>

                <Row spacing={10}>
                    <Button onClick={() => Audio.play("sound.mp3")}>
                        Play Audio
                    </Button>
                    <Button onClick={() => Audio.stop()}>
                        Stop Audio
                    </Button>
                </Row>
            </Column>
        </Column>
    );
}

