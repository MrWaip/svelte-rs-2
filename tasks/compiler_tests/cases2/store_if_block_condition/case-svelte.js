import * as $ from "svelte/internal/client";
import { cond } from "./stores";
var root = $.from_html(`<p>visible</p>`);
export default function App($$anchor) {
	const $cond = () => $.store_get(cond, "$cond", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($cond()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$$cleanup();
}
