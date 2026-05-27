<svelte:options runes={true} />

<script>
    const transform = "translateY(1px)";
    let color = $state("red");
</script>

<div style:transform={transform} style:color={color}></div>
<button onclick={() => color = "blue"}>change</button>
