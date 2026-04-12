<script>
	let props = { foo: 'bar' };
	let item = 'hello';
	let extra = 'world';
</script>

<slot name="footer" item={item} {...props} extra={extra} />
