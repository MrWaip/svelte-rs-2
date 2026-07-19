App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
const $$css = {
	hash: "svelte-p6hspt",
	code: "\n	.icon.svelte-p6hspt::before {\n		content: \"\\ff\";\n	}\n\n/*# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiKHVua25vd24pIiwic291cmNlcyI6WyIodW5rbm93bikiXSwic291cmNlc0NvbnRlbnQiOlsiPHN2ZWx0ZTpvcHRpb25zIGNzcz1cImluamVjdGVkXCIgLz5cblxuPHN0eWxlPlxuXHQuaWNvbjo6YmVmb3JlIHtcblx0XHRjb250ZW50OiBcIlxcZmZcIjtcblx0fVxuPC9zdHlsZT5cblxuPHNwYW4gY2xhc3M9XCJpY29uXCI+PC9zcGFuPlxuIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiI7QUFHQSxDQUFDLG1CQUFLLFFBQVEsQ0FBQztBQUNmLEVBQUUsY0FBYztBQUNoQjsifQ== */"
};
function App($$renderer, $$props) {
	$$renderer.global.css.add($$css);
	$$renderer.component(($$renderer) => {
		$$renderer.push(`<span class="icon svelte-p6hspt">`);
		$.push_element($$renderer, "span", 9, 0);
		$$renderer.push(`</span>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
