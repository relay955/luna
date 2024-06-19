<script lang="ts">
  import LeftItemList from "./LeftItemList.svelte";
  import TitleBar from "./TitleBar.svelte";
  import TopMenu from "./TopMenu.svelte";

  export let searchbarMode : "filter"|"search"|"path" = "path";
</script>

<div class="container">
  <TitleBar />
  <div style="margin-top: 30px;"></div>
  <TopMenu searchbarMode={searchbarMode} />
  <div class="inner-container">
    <LeftItemList />
    <div class="file-list">
      <div>나는 페이지!2</div>
      <div>나는 페이지!3</div>
      <div>나는 페이지!4</div>
      <div>나는 페이지!5</div>
      <div>나는 페이지!</div>
      <div>나는 페이지!</div>
      <div>나는 페이지!</div>
      <div>나는 페이지!</div>
      <div>나는 페이지!</div>
    </div>
  </div>
</div>

<style>
  .container{
    height:100%;
    display: flex;
    flex-direction: column;
  }
  .inner-container {
    width: 100%;
    height: 100%;
    display: flex;
  }
  .file-list{
    flex-grow: 1;
  }
</style>
