<script>
	import { page } from '$app/stores';
</script>

<div class="text-center">
  <h1 class="text-3xl mb-3">{$page.status} {$page.error?.message}</h1>
  <span class="text-9xl">
    💥
  </span>
</div>