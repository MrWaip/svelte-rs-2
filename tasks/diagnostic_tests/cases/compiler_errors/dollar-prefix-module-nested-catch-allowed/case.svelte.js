export function outer() {
	function inner() {
		try {
			fetch('/');
		} catch ($$error) {
			console.log($$error);
		}
	}
	return inner;
}
