import * as $ from "svelte/internal/client";
import { SvelteSet } from "svelte/reactivity";
var root = $.from_html(`<button>add</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const set = new SvelteSet();
	var button = root();
	$.delegated("click", button, () => {
		const s = $.proxy({ x: 1 });
		set.add(s);
	});
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
