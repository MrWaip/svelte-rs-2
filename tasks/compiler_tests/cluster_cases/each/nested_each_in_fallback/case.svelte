<script>let a = $state([]); let b = $state([]);</script>
{#each a as x}<p>{x}</p>{:else}{#each b as x}<span>{x}</span>{/each}{/each}
