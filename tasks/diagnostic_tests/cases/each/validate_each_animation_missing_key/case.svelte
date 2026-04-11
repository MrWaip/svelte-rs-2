<script>import { flip } from 'svelte/animate'; let items = [];</script>
{#each items as item}
    <div animate:flip>{item}</div>
{/each}
