<div>
	{let dt = $derived([1, 2, 3])}
	{dt.length}
	{dt.map((x) => x + dt.length)}
</div>
