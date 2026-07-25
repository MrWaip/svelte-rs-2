export const outer = () => (() => {
	try {
		fetch('/');
	} catch ($$error) {
		console.log($$error);
	}
});
