import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let todos = [{ done: false }];
	function update() {
		todos = todos.slice(1);
	}
	$$renderer.push(`<button>x</button> <!--[-->`);
	const each_array = $.ensure_array_like(todos);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let todo = each_array[$$index];
		$$renderer.push(`<input type="checkbox"${$.attr("checked", todo.done, true)}/>`);
	}
	$$renderer.push(`<!--]-->`);
}
