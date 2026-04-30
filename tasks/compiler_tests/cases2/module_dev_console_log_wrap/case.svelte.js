let count = $state(0);

export function increment() {
	count++;
}

$effect(() => {
	console.log("count changed:", count);
});
