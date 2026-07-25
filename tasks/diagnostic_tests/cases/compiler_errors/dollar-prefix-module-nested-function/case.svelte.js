export function outer() {
	function inner() {
		let $$a = 1;
		return $$a;
	}
	return inner;
}
