App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
var root = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.tag($.state(0), "value");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.child(div);
	var bind_get = () => $.get(value);
	var bind_set = (v) => $.set(value, v, true);
	$.add_svelte_meta(() => Comp(node, {
		get value() {
			return bind_get();
		},
		set value($$value) {
			bind_set($$value);
		}
	}), "component", App, 7, 1, { componentTag: "Comp" });
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
