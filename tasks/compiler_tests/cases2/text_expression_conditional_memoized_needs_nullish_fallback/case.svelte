<script>
	let { card } = $props();
	function fmt(x) { return x; }
</script>
<span>{card.flag ? 'A' : 'B' + fmt(card.x)} • {card.tail}</span>
