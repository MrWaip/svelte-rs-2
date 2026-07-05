App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	let a = 1;
	let b = 2;
	$.inspect(() => [count], (...$$args) => console.log(...$$args), true);
	$.inspect(() => [a, b], (...$$args) => console.log(...$$args), true);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
