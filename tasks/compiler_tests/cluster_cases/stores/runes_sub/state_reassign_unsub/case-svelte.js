import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root_1 = $.from_html(` <button>remove</button>`, 1);
var root_2 = $.from_html(`<button>add</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $watcherA = () => $.store_get($.get(watcherA), "$watcherA", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let watcherA = $.state(void 0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root_1();
			var text = $.first_child(fragment_1);
			var button = $.sibling(text);
			$.template_effect(() => $.set_text(text, `${$watcherA() ?? ""} `));
			$.delegated("click", button, () => $.store_unsub($.set(watcherA, null), "$watcherA", $$stores));
			$.append($$anchor, fragment_1);
		};
		var alternate = ($$anchor) => {
			var button_1 = root_2();
			$.delegated("click", button_1, () => $.store_unsub($.set(watcherA, writable(0), true), "$watcherA", $$stores));
			$.append($$anchor, button_1);
		};
		$.if(node, ($$render) => {
			if ($.get(watcherA)) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
