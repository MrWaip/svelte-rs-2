import { make } from './ctx.svelte';

type Pair = [number, number];

export const factory = make('store', (props: { x: number }) => {
    let items: string[] = $state([]);

    function add(id: string) {
        items.push(id);
    }

    function remove(id: string) {
        items = items.filter((v) => v !== id);
    }

    const count = $derived(items.length);

    const summary = $derived.by((): Pair => {
        if (count === 0) return [0, 0];
        return [props.x, count];
    });

    return {
        add,
        remove,
        get count() {
            return count;
        },
        get summary() {
            return summary;
        },
    };
});
