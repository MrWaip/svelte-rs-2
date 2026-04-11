<script>import { flip, crossfade } from 'svelte/animate'; let items = [];</script>
{#each items as item (item.id)}
    <div animate:flip animate:crossfade>{item}</div>
{/each}
