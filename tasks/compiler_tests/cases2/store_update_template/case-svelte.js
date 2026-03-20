import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.from_html(`<button>inc</button> <button>pre inc</button> <button>dec</button>`, 1);
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var button_2 = $.sibling(button_1, 2);
	$.delegated("click", button, () => $.update_store(count, $count()));
	$.delegated("click", button_1, () => $.update_pre_store(count, $count()));
	$.delegated("click", button_2, () => $.update_store(count, $count(), -1));
	$.append($$anchor, fragment);
	$$cleanup();
}
$.delegate(["click"]);
