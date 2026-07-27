export function outer() {
	function inner() {
		function $$deep() {}
		return $$deep;
	}
	return inner;
}
