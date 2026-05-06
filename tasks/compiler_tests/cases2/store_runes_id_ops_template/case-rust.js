import * as $ from "svelte/internal/client";
import { count } from "./stores";
var root = $.from_html(`<button>set</button> <button>+=</button> <button>??=</button> <button>&&=</button> <button>||=</button> <button>++</button> <button>--pre</button>`, 1);
export default function App($$anchor) {
	const $count = () => $.store_get(count, "$count", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var button_2 = $.sibling(button_1, 2);
	var button_3 = $.sibling(button_2, 2);
	var button_4 = $.sibling(button_3, 2);
	var button_5 = $.sibling(button_4, 2);
	var button_6 = $.sibling(button_5, 2);
	$.delegated("click", button, () => $.store_set(count, 1));
	$.delegated("click", button_1, () => $.store_set(count, $count() + 1));
	$.delegated("click", button_2, () => $.store_set(count, $count() ?? 5));
	$.delegated("click", button_3, () => $.store_set(count, $count() && 5));
	$.delegated("click", button_4, () => $.store_set(count, $count() || 5));
	$.delegated("click", button_5, () => $.update_store(count, $count()));
	$.delegated("click", button_6, () => $.update_pre_store(count, $count(), -1));
	$.append($$anchor, fragment);
	$$cleanup();
}
$.delegate(["click"]);
