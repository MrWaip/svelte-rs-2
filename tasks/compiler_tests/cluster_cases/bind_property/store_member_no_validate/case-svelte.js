import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<input type="number"/>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $obj = () => $.store_get(obj, "$obj", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const obj = writable({ a: 1 });
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, () => $obj().a, ($$value) => $.store_mutate(obj, $.untrack($obj).a = $$value, $.untrack($obj)));
	$.append($$anchor, input);
	$.pop();
	$$cleanup();
}
