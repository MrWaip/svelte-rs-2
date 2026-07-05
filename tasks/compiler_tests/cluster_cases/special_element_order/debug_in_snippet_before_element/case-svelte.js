import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>+</button>`);
export default function App($$anchor) {
	const row = ($$anchor) => {
		var button = root();
		$.template_effect(() => {
			console.log({ count: $.snapshot($.get(count)) });
			debugger;
		});
		$.delegated("click", button, () => $.update(count));
		$.append($$anchor, button);
	};
	let count = $.state(0);
	row($$anchor);
}
$.delegate(["click"]);
