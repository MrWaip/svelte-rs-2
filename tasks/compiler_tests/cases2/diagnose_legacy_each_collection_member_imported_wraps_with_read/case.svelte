<svelte:options runes={false} />
<script>
    import { LINKS } from './links.js';
    export let title = '';
    $: heading = title.toUpperCase();
</script>

<h1>{heading}</h1>
{#each LINKS.list as link}
    <a href={link.href}>{link.label}</a>
{/each}
