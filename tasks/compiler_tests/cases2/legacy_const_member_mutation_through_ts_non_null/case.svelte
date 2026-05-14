<svelte:options runes={false} />

<script lang="ts">
    type Lazy = { data: { selected: string | null } | null };

    function loader(): Lazy {
        return { data: { selected: null } };
    }

    const state = loader();

    function reset() {
        if (state.data === null) return;
        state.data!.selected = null;
    }

    function pick(value: string) {
        state.data!.selected = value;
    }
</script>

{#if state.data}
    <button onclick={reset}>{state.data.selected}</button>
{/if}
<button onclick={() => pick('x')}>x</button>
