import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let show = true;
	let x = 42;
	let items = [
		1,
		2,
		3
	];
	if (show) {
		$$renderer.push("<!--[0-->");
		console.log({ x });
		debugger;
	} else {
		$$renderer.push("<!--[-1-->");
	}
	$$renderer.push(`<!--]--> <!--[-->`);
	const each_array = $.ensure_array_like(items);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let item = each_array[$$index];
		console.log({ item });
		debugger;
	}
	$$renderer.push(`<!--]--> <div>`);
	console.log({ x });
	debugger;
	$$renderer.push(`<p>Value: 42</p></div>`);
}
