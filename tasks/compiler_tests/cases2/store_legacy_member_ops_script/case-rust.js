import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { obj } from "./stores";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $obj = () => $.store_get(obj, "$obj", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	function go() {
		$.store_mutate(obj, $.untrack($obj).x = 1, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj).x += 1, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj).x ??= 5, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj).x++, $.untrack($obj));
		$.store_mutate(obj, ++$.untrack($obj).x, $.untrack($obj));
	}
	$.init();
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
