App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<p class="svelte-sw3owg"> </p>`), App[$.FILENAME], [[13, 0]]);
const $$css = {
	hash: "svelte-sw3owg",
	code: "\n    p.svelte-sw3owg { color: red; }\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHNjcmlwdD5cbiAgICBpbXBvcnQgeyB3cml0YWJsZSB9IGZyb20gXCJzdmVsdGUvc3RvcmVcIjtcbiAgICBsZXQgc3RvcmUgPSB3cml0YWJsZSgwKTtcbiAgICBsZXQgY291bnQgPSAkc3RhdGUoMCk7XG48L3NjcmlwdD5cblxuPHN0eWxlPlxuICAgIHAgeyBjb2xvcjogcmVkOyB9XG48L3N0eWxlPlxuXG48cD57JHN0b3JlfSB7Y291bnR9PC9wPlxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiI7QUFTQSxJQUFJLGVBQUMsQ0FBQyxFQUFFLFVBQVU7In0= */"
};
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	$.append_styles($$anchor, $$css);
	const $store = () => ($.validate_store(store, "store"), $.store_get(store, "$store", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = writable(0);
	let count = 0;
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$store() ?? ""} 0`));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
