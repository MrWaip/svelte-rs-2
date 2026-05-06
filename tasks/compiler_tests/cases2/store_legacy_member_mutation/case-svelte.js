import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { obj } from "./stores";
var root = $.from_html(`<button>++</button> <button>=</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $obj = () => $.store_get(obj, "$obj", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	$.init();
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	$.delegated("click", button, () => $.store_mutate(obj, $.untrack($obj).value++, $.untrack($obj)));
	$.delegated("click", button_1, () => $.store_mutate(obj, $.untrack($obj).value = 1, $.untrack($obj)));
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
