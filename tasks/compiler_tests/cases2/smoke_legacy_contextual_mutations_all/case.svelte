<svelte:options runes={false} />
<script>
	import { writable } from 'svelte/store';

	export let count = 0;
	export let obj = { x: 0 };

	let local = 0;
	let localObj = { x: 0 };
	let key = 'x';

	const store = writable(0);
	const objStore = writable({ x: 0 });

	let items = [{ id: 1, x: 0 }, { id: 2, x: 0 }];
	let promise = Promise.resolve({ x: 0 });

	function script_ops() {
		count = 1;
		count += 2;
		count -= 3;
		count++;
		count--;
		++count;
		--count;
		count &&= 4;
		count ||= 5;
		count ??= 6;

		obj.x = 7;
		obj.x += 8;
		obj.x++;
		obj.x--;
		++obj.x;
		--obj.x;
		obj.x &&= 9;
		obj.x ||= 10;
		obj.x ??= 11;
		obj["x"] = 7;
		obj["x"] += 8;
		obj["x"]++;
		++obj["x"];
		obj[key] = 7;
		obj[key] += 8;
		obj[key]++;
		++obj[key];

		local = 12;
		local += 13;
		local -= 14;
		local++;
		local--;
		++local;
		--local;
		local &&= 15;
		local ||= 16;
		local ??= 17;

		localObj.x = 18;
		localObj.x += 19;
		localObj.x++;
		localObj.x--;
		++localObj.x;
		--localObj.x;
		localObj.x &&= 20;
		localObj.x ||= 21;
		localObj.x ??= 22;
		localObj["x"] = 18;
		localObj["x"] += 19;
		localObj["x"]++;
		++localObj["x"];
		localObj[key] = 18;
		localObj[key] += 19;
		localObj[key]++;
		++localObj[key];

		$store = 23;
		$store += 24;
		$store -= 25;
		$store++;
		$store--;
		++$store;
		--$store;
		$store &&= 26;
		$store ||= 27;
		$store ??= 28;

		$objStore.x = 29;
		$objStore.x += 30;
		$objStore.x++;
		$objStore.x--;
		++$objStore.x;
		--$objStore.x;
		$objStore.x &&= 31;
		$objStore.x ||= 32;
		$objStore.x ??= 33;
		$objStore["x"] = 29;
		$objStore["x"] += 30;
		$objStore["x"]++;
		++$objStore["x"];
		$objStore[key] = 29;
		$objStore[key] += 30;
		$objStore[key]++;
		++$objStore[key];
	}
</script>

{count}
{obj.x}
{local}
{localObj.x}
{$store}
{$objStore.x}

{count = 1}
{count += 2}
{count -= 3}
{count++}
{count--}
{++count}
{--count}
{count &&= 4}
{count ||= 5}
{count ??= 6}

{obj.x = 7}
{obj.x += 8}
{obj.x++}
{obj.x--}
{++obj.x}
{--obj.x}
{obj.x &&= 9}
{obj.x ||= 10}
{obj.x ??= 11}
{obj["x"] = 7}
{obj["x"] += 8}
{obj["x"]++}
{++obj["x"]}
{obj[key] = 7}
{obj[key] += 8}
{obj[key]++}
{++obj[key]}

{local = 12}
{local += 13}
{local -= 14}
{local++}
{local--}
{++local}
{--local}
{local &&= 15}
{local ||= 16}
{local ??= 17}

{localObj.x = 18}
{localObj.x += 19}
{localObj.x++}
{localObj.x--}
{++localObj.x}
{--localObj.x}
{localObj.x &&= 20}
{localObj.x ||= 21}
{localObj.x ??= 22}
{localObj["x"] = 18}
{localObj["x"] += 19}
{localObj["x"]++}
{++localObj["x"]}
{localObj[key] = 18}
{localObj[key] += 19}
{localObj[key]++}
{++localObj[key]}

{$store = 23}
{$store += 24}
{$store -= 25}
{$store++}
{$store--}
{++$store}
{--$store}
{$store &&= 26}
{$store ||= 27}
{$store ??= 28}

{$objStore.x = 29}
{$objStore.x += 30}
{$objStore.x++}
{$objStore.x--}
{++$objStore.x}
{--$objStore.x}
{$objStore.x &&= 31}
{$objStore.x ||= 32}
{$objStore.x ??= 33}
{$objStore["x"] = 29}
{$objStore["x"] += 30}
{$objStore["x"]++}
{++$objStore["x"]}
{$objStore[key] = 29}
{$objStore[key] += 30}
{$objStore[key]++}
{++$objStore[key]}

{#each items as item, i (item.id)}
	{item}
	{i}
	{item.x}
	{item.x = 1}
	{item.x += 2}
	{item.x -= 3}
	{item.x++}
	{item.x--}
	{++item.x}
	{--item.x}
	{item.x &&= 4}
	{item.x ||= 5}
	{item.x ??= 6}
	{item["x"] = 1}
	{item["x"] += 2}
	{item["x"]++}
	{++item["x"]}
	{item[key] = 1}
	{item[key] += 2}
	{item[key]++}
	{++item[key]}
	<button onclick={() => {
		item.x = 1;
		item.x += 2;
		item.x++;
		++item.x;
		item.x &&= 4;
		item.x ||= 5;
		item.x ??= 6;
		item["x"] = 1;
		item[key] = 1;
		item[key]++;
	}}>each-row</button>
{/each}

{#snippet card(param)}
	{param}
	{param.x}
	{param.x = 1}
	{param.x += 2}
	{param.x++}
	{++param.x}
	{param.x &&= 3}
	{param.x ||= 4}
	{param.x ??= 5}
	{param["x"] = 1}
	{param["x"]++}
	{param[key] = 1}
	{param[key]++}
	<button onclick={() => {
		param.x = 1;
		param.x += 2;
		param.x++;
		++param.x;
		param.x &&= 3;
		param.x ||= 4;
		param.x ??= 5;
		param[key] = 1;
		param[key]++;
	}}>snippet</button>
{/snippet}

{#each items as it (it.id)}
	{@const ctx = it}
	{ctx.x}
	{ctx.x = 1}
	{ctx.x += 2}
	{ctx.x++}
	{++ctx.x}
	{ctx.x &&= 3}
	{ctx.x ||= 4}
	{ctx.x ??= 5}
	{ctx["x"] = 1}
	{ctx[key] = 1}
{/each}

{#await promise then v}
	{v.x}
	{v.x = 1}
	{v.x += 2}
	{v.x++}
	{++v.x}
	{v.x &&= 3}
	{v.x ||= 4}
	{v.x ??= 5}
	{v["x"] = 1}
	{v[key] = 1}
{:catch e}
	{e.message}
	{e.x = 1}
	{e.x += 2}
	{e.x++}
	{++e.x}
	{e["x"] = 1}
	{e[key] = 1}
{/await}

<button onclick={script_ops}>run</button>
