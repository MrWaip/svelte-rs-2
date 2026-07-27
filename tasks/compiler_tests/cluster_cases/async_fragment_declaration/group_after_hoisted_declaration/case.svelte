<script>const id = 'name';</script>
<div>
	{const nested = 'nested'}
	{const greeting2 = $derived(await `Hi ${id}`)}
	<span>{nested} {greeting2}</span>
</div>
