import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const foo = ($$anchor) => {
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, $count()));
		$.append($$anchor, text);
	};
	let count = writable(0);
	count = writable(1);
	foo($$anchor);
	$.pop();
	$$cleanup();
}
