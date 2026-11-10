import { Column, View, Text, TextInput, Button } from "ferrix/ui";
import { useForm, required, email, minLength } from "ferrix/forms";

export function LoginForm(): Element {
    const form = useForm({
        email: { initial: "", validators: [required("Email is required"), email("Must be a valid email")] },
        password: { initial: "", validators: [required("Password is required"), minLength(8, "Minimum 8 characters")] },
    });

    const onSubmit = () => {
        form.handleSubmit(() => {
            // Authenticate user
        });
    };

    return (
        <Column style={{ flex: 1, padding: 24, justifyContent: "center", backgroundColor: "#0f172a" }}>
            <Text style={{ fontSize: 24, fontWeight: "bold", color: "#f8fafc", marginBottom: 24 }}>
                Account Sign In
            </Text>

            <View style={{ marginBottom: 16 }}>
                <Text style={{ fontSize: 14, color: "#94a3b8", marginBottom: 6 }}>Email Address</Text>
                <TextInput
                    value={form.fields.email.value}
                    onChangeText={(text) => form.fields.email.setValue(text)}
                    onBlur={() => form.fields.email.setTouched(true)}
                    placeholder="user@example.com"
                    style={{ backgroundColor: "#1e293b", padding: 12, borderRadius: 8, color: "#ffffff" }}
                />
                {form.fields.email.error && (
                    <Text style={{ fontSize: 12, color: "#ef4444", marginTop: 4 }}>
                        {form.fields.email.error}
                    </Text>
                )}
            </View>

            <View style={{ marginBottom: 24 }}>
                <Text style={{ fontSize: 14, color: "#94a3b8", marginBottom: 6 }}>Password</Text>
                <TextInput
                    value={form.fields.password.value}
                    onChangeText={(text) => form.fields.password.setValue(text)}
                    onBlur={() => form.fields.password.setTouched(true)}
                    secureTextEntry={true}
                    placeholder="••••••••"
                    style={{ backgroundColor: "#1e293b", padding: 12, borderRadius: 8, color: "#ffffff" }}
                />
                {form.fields.password.error && (
                    <Text style={{ fontSize: 12, color: "#ef4444", marginTop: 4 }}>
                        {form.fields.password.error}
                    </Text>
                )}
            </View>

            <Button
                title={form.isSubmitting ? "Signing in..." : "Sign In"}
                onPress={onSubmit}
            />
        </Column>
    );
}

