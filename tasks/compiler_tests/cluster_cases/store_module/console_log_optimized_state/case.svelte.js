const value = $state(0);

$effect.root(() => {
	$effect(() => {
		console.log(value);
	});
});
