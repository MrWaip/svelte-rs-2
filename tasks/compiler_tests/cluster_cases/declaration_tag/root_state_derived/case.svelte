{let count = $state(0)}
{let double = $derived(count * 2)}
<button onclick={() => count++}>{count} {double}</button>
