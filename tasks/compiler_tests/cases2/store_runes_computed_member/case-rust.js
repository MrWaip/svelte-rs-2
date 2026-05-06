import * as $ from "svelte/internal/client";
import { obj } from "./stores";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $obj = () => $.store_get(obj, "$obj", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let key = "x";
	function go() {
		$.store_mutate(obj, $.untrack($obj)["k"] = 1, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj)[key] = 2, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj)["k"] += 1, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj)[key]++, $.untrack($obj));
		$.store_mutate(obj, $.untrack($obj)["k"] ??= 5, $.untrack($obj));
	}
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
