<script>
  import Banner from './Banner.svelte';
  import { cardHeight } from '$lib/store';

  export let title = '';
  export let banner = '';
  export let selected = false;
</script>

<div on:click class="card-wrapper" class:selected>
  <div class="card">
    <div class="img-container" style="padding-top: calc(({$cardHeight})*100%)">
      <Banner absolute={true} rounded {banner} fallbackText={title} />
    </div>
  </div>
</div>

<style lang="postcss">
  .img-container {
    position: relative;
    width: 100%;
  }

  .card-wrapper {
    border: 2px solid var(--secondary);
    border-radius: 10px;
    background-color: transparent;
    transition: border-color 0.2s;
  }

  .card {
  }

  .card-wrapper.selected,
  .card-wrapper:hover {
    border-color: var(--primary);
    box-shadow: 0px 0px 10px var(--primary);
  }
</style>
