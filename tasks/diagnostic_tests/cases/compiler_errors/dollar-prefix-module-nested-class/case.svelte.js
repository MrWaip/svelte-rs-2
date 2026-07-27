export function outer() {
	function inner() {
		class $$C {}
		return $$C;
	}
	return inner;
}
