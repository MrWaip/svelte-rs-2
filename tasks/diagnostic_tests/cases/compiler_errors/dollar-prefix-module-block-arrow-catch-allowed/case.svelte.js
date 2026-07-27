export const outer = () => {
	const inner = () => {
		try {
			fetch('/');
		} catch ($$error) {
			console.log($$error);
		}
	};
	return inner;
};
