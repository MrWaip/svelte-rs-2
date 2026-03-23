import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<h2></h2>`);
export default function App($$anchor) {
	let count = 0;
	{
		const header = ($$anchor) => {
			var h2 = root_1();
			h2.textContent = "Title 0";
			$.append($$anchor, h2);
		};
		Dialog($$anchor, {
			header,
			$$slots: { header: true }
		});
	}
}
