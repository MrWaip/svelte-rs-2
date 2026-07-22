const state = $state(0);
let inspect = {};

$effect.root(() => {
	$inspect(state);
});
