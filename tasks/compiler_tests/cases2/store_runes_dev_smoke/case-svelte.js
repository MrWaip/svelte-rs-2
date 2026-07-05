import * as $ from "svelte/internal/client";
import { obj, count } from "./stores";
var root = $.from_html(`<p> </p> <p> </p> <button>go</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $count = () => $.store_get(count, "$count", $$stores);
	const $obj = () => $.store_get(obj, "$obj", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	function go() {
		$.store_set(count, 1);
		$.update_store(count, $count());
		$.store_mutate(obj, $.untrack($obj).x = 1, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj).x++, $.untrack($obj));
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	var button = $.sibling(p_1, 2);
	$.template_effect(() => {
		$.set_text(text, $count());
		$.set_text(text_1, $obj().x);
	});
	$.delegated("click", button, go);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
