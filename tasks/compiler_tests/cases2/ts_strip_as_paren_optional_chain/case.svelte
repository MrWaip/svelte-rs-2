<svelte:options runes={false} />
<script lang="ts">
    type Foo = { y: number };
    export let obj: { x?: Foo } | undefined;
    const z = (obj?.x as Foo)?.y;
</script>
<p>{z}</p>
