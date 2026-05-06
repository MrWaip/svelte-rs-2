import * as $ from "svelte/internal/client";
import { obj } from "./stores";
var root = $.from_html(`<button>set</button> <button>+=</button> <button>??=</button> <button>++</button> <button>--pre</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $obj = () => $.store_get(obj, "$obj", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var button_2 = $.sibling(button_1, 2);
	var button_3 = $.sibling(button_2, 2);
	var button_4 = $.sibling(button_3, 2);
	$.delegated("click", button, () => $.store_mutate(obj, $.untrack($obj).x = 1, $.untrack($obj)));
	$.delegated("click", button_1, () => $.store_mutate(obj, $.untrack($obj).x += 1, $.untrack($obj)));
	$.delegated("click", button_2, () => $.store_mutate(obj, $.untrack($obj).x ??= 1, $.untrack($obj)));
	$.delegated("click", button_3, () => $.store_mutate(obj, $.untrack($obj).x++, $.untrack($obj)));
	$.delegated("click", button_4, () => $.store_mutate(obj, --$.untrack($obj).x, $.untrack($obj)));
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
