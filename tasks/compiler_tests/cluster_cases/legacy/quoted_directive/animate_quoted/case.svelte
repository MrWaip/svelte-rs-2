<script>function flip(node){ return {}; }
let items = [1, 2, 3];</script>
{#each items as item (item)}<div animate:flip="{{ duration: 200 }}">{item}</div>{/each}