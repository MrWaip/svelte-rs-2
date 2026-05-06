import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.from_html(`<button>=</button> <button>+=</button> <button>??=</button> <button>++</button>`, 1);
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var button_2 = $.sibling(button_1, 2);
	var button_3 = $.sibling(button_2, 2);
	$.delegated("click", button, () => $.store_set(count, 1));
	$.delegated("click", button_1, () => $.store_set(count, $count() + 1));
	$.delegated("click", button_2, () => $.store_set(count, $count() ?? 5));
	$.delegated("click", button_3, () => $.update_store(count, $count()));
	$.append($$anchor, fragment);
	$$cleanup();
}
$.delegate(["click"]);
