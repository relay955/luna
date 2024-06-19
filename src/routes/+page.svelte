<script lang="ts">
  import LeftItemList from "./LeftItemList.svelte";
    import ListView from "./ListView.svelte";
  import TitleBar from "./TitleBar.svelte";
  import TopMenu from "./TopMenu.svelte";

  let searchbarMode : "filter"|"search"|"path" = "path";
  let directory = "c:/";
</script>

<div class="container">
  <TitleBar />
  <div style="margin-top: 30px;"></div>
  <TopMenu searchbarMode={searchbarMode} />
  <div class="inner-container">
    <LeftItemList />
    <div class="file-list">
      <ListView/>
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
