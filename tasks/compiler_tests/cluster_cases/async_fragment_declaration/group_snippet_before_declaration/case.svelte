<script>const id = "name";</script>
<div>
{#snippet greet(x)}<b>{x}</b>{/snippet}
{const greeting2 = $derived(await `Hi ${id}`)}
<span>{greeting2}</span>
{@render greet(1)}
</div>
