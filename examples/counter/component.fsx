import { Column, Text, Button } from "ferrix/ui";

type CounterProps = { initial: i32 };

export function Counter(props: CounterProps): Element {
    const count = useState(props.initial);

    return (
        <Column padding={20} spacing={12}>
            <Text size={24}>Ferrix Counter</Text>
            <Text>Count: {count.value}</Text>
            <Button onClick={() => count.set(count.value + 1)}>
                Increment
            </Button>
        </Column>
    );
}

