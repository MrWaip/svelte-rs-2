import * as $ from "svelte/internal/client";
var root = $.from_html(`<b>hi</b>`);
export default function App($$anchor) {
	{
		const foo = ($$anchor) => {
			var b = root();
			$.append($$anchor, b);
		};
		Component($$anchor, {
			foo,
			$$slots: { foo: true }
		});
	}
}
