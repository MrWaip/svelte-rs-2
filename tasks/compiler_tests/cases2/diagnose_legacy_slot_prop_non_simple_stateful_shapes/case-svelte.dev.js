import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8, 0);
	let b = $.prop($$props, "b", 8, 0);
	let flag = $.prop($$props, "flag", 8, false);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "default", {
		get sum() {
			return a() + b();
		},
		get neg() {
			return !flag();
		},
		get both() {
			return a() && b();
		},
		get cond() {
			return flag() ? a() : b();
		}
	}, null);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
