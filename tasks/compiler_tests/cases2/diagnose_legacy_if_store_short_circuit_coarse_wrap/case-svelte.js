import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $items = () => $.store_get(items, "$items", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let prop = $.prop($$props, "prop", 8);
	const items = writable([]);
	let show = true;
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root_1();
			var text = $.child(p);
			$.reset(p);
			$.template_effect(() => $.set_text(text, `shown ${prop() ?? ""}`));
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($items(), $.untrack(() => show && $items().length > 0)) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
