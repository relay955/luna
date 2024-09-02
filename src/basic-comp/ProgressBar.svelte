<script lang="ts">
    export let value:number;
    export let height:number = 10;
</script>

<div class="container" style="height:{height}px">
    <div class="progressbar-background" style="width: 100%"/>
    <div class="progressbar-foreground" style="width: {value}%"/>
</div>

<style lang="scss">
  .container {
    position: relative;
  }

  .progressbar-background {
    position: absolute;
    top: 0;
    left: 0;
    background-color: #f0f0f0;
    height: 100%;
  }

  .progressbar-foreground {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 1;
    background-color: #007bff;
    height: 100%;
  }
</style>
