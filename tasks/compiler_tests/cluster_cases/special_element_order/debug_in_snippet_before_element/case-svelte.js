import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button>+</button>`);
export default function App($$anchor) {
	const row = ($$anchor) => {
		var button = root_1();
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
