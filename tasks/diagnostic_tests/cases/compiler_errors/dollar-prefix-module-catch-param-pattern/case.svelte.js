export function run() {
	try {
		fetch('/');
	} catch ({ $$message }) {
		console.log($$message);
	}
}
