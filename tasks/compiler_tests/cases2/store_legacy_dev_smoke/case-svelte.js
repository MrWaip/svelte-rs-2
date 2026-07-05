import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { count, obj } from "./stores";
var root = $.from_html(`<p> </p> <p> </p> <button>go</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $count = () => $.store_get(count, "$count", $$stores);
	const $obj = () => $.store_get(obj, "$obj", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	function go() {
		$.store_set(count, 1);
		$.update_store(count, $count());
		$.store_mutate(obj, $.untrack($obj).x = 1, $.untrack($obj));
	}
	$.init();
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
		$.set_text(text_1, ($obj(), $.untrack(() => $obj().x)));
	});
	$.delegated("click", button, go);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
