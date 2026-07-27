<script>let { a } = $props(); let g = $state();</script>
<input type="checkbox" bind:checked={g} value={await a} />
