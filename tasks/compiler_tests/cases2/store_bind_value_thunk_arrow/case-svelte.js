import * as $ from "svelte/internal/client";
import { name } from "./stores";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	const $name = () => $.store_get(name, "$name", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var input = root();
	$.remove_input_defaults(input);
	$.bind_value(input, $name, ($$value) => $.store_set(name, $$value));
	$.append($$anchor, input);
	$$cleanup();
}
