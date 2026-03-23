import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<h1>Header</h1>`);
var root_2 = $.from_html(`<p>Footer</p>`);
export default function App($$anchor) {
	{
		const header = ($$anchor) => {
			var h1 = root_1();
			$.append($$anchor, h1);
		};
		const footer = ($$anchor) => {
			var p = root_2();
			$.append($$anchor, p);
		};
		Card($$anchor, {
			header,
			footer,
			$$slots: {
				header: true,
				footer: true
			}
		});
	}
}
