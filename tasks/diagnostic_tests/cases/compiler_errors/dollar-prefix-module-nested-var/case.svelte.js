export function outer() {
	function inner() {
		var $$a = 1;
		return $$a;
	}
	return inner;
}
