export const make = () => {
	let x = $state(0);
	const s = $derived(x + 1);
	return s;
};
