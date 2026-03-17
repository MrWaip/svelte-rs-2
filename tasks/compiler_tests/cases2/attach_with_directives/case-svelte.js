import * as $ from "svelte/internal/client";
import { tooltip } from "./actions.js";
import { fade } from "svelte/transition";
var root = $.from_html(`<input/>`);
export default function App($$anchor) {
	let value = $.state("");
	var input = root();
	$.remove_input_defaults(input);
	$.attach(input, () => tooltip);
	$.bind_value(input, () => $.get(value), ($$value) => $.set(value, $$value));
	$.transition(3, input, () => fade);
	$.append($$anchor, input);
}
