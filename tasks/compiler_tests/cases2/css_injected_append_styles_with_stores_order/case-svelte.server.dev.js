App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
const $$css = {
	hash: "svelte-sw3owg",
	code: "\n    p.svelte-sw3owg { color: red; }\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHNjcmlwdD5cbiAgICBpbXBvcnQgeyB3cml0YWJsZSB9IGZyb20gXCJzdmVsdGUvc3RvcmVcIjtcbiAgICBsZXQgc3RvcmUgPSB3cml0YWJsZSgwKTtcbiAgICBsZXQgY291bnQgPSAkc3RhdGUoMCk7XG48L3NjcmlwdD5cblxuPHN0eWxlPlxuICAgIHAgeyBjb2xvcjogcmVkOyB9XG48L3N0eWxlPlxuXG48cD57JHN0b3JlfSB7Y291bnR9PC9wPlxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiI7QUFTQSxJQUFJLGVBQUMsQ0FBQyxFQUFFLFVBQVU7In0= */"
};
function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let store = writable(0);
		let count = 0;
		$$renderer.push(`<p class="svelte-sw3owg">`);
		$.push_element($$renderer, "p", 13, 0);
		$$renderer.push(`${$.escape($.store_get($$store_subs ??= {}, "$store", store))} 0</p>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
