import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>increment</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $count = () => $.store_get($.get(count), "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let count = $.state(0);
	$.user_effect(() => {
		$.store_set($.get(count), 1);
	});
	var button = root();
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
