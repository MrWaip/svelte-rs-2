<script>import { flip } from 'svelte/animate'; let items = [];</script>
{#each items as item (item.id)}
    <div animate:flip>{item}</div>
    <span>extra</span>
{/each}
