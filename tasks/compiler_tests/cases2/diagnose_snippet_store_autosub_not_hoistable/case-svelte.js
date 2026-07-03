import * as $ from "svelte/internal/client";
import { page } from "$app/stores";
const defaultWrapWith = ($$anchor, mf = $.noop) => {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.snippet(node, mf);
	$.append($$anchor, fragment);
};
var root = $.from_html(`<b>x</b>`);
var root_1 = $.from_html(`<div><!></div>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $page = () => $.store_get(page, "$page", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const mf = ($$anchor) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				var b = root();
				$.append($$anchor, b);
			};
			$.if(node_1, ($$render) => {
				if ($page().url) $$render(consequent);
			});
		}
		$.append($$anchor, fragment_1);
	};
	let wrapWith = $.prop($$props, "wrapWith", 3, defaultWrapWith);
	var div = root_1();
	var node_2 = $.child(div);
	$.snippet(node_2, wrapWith, () => mf);
	$.reset(div);
	$.append($$anchor, div);
	$.pop();
	$$cleanup();
}
