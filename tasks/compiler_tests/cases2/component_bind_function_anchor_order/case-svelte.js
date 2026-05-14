import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
var root = $.from_html(`<div><!></div>`);
export default function App($$anchor) {
	let value = $.state(0);
	var div = root();
	var node = $.child(div);
	var bind_get = () => $.get(value);
	var bind_set = (v) => $.set(value, v, true);
	Comp(node, {
		get value() {
			return bind_get();
		},
		set value($$value) {
			bind_set($$value);
		}
	});
	$.reset(div);
	$.append($$anchor, div);
}
