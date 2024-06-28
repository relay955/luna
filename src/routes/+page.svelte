<script lang="ts">
    import type { FileItem } from "../logics/fileitem";
    import {
      generateIconCacheUpdateReq,
      type IconCache,
      mergeIconCacheFromUpdateRes
    } from "../logics/iconcache";
  import LeftItemList from "./LeftItemList.svelte";
    import ListView from "./ListView.svelte";
  import TitleBar from "./TitleBar.svelte";
  import TopMenu from "./TopMenu.svelte";
  import { invoke } from "@tauri-apps/api/tauri"
    import {onMount} from "svelte";
    import type {DriveInfo} from "../logics/driveinfo";

  let searchbarMode : "filter"|"search"|"path" = "path";
  let directory = "c:/";
  let fileItems:FileItem[] = [];
  let directoryHistory:string[] = [];
  let driveList:DriveInfo[] = [];

  let iconCache:IconCache = {
    ext: {},
    file: {},
    folderIcon: null
  };

  onMount(()=>{
    invoke("get_drive_list").then((res) => {
      driveList = res as DriveInfo[];
    });
  });

  $:directory, onChangeDirectory()
  const onChangeDirectory = () =>{
    let options = {
      dir:directory,
      sortBy:"name",
      sortDirection:"asc",
      groupingMode:"folder",
      search:"",
      filter:""
    }
    invoke("get_file_list", options).then(async (res) => {
      fileItems = res as FileItem[];
      let {req, reqTypeByFile} = generateIconCacheUpdateReq(iconCache, res as FileItem[]);
      let updateIconCacheRes = await invoke("get_icons", {req: req});
      iconCache = mergeIconCacheFromUpdateRes(iconCache, reqTypeByFile,updateIconCacheRes as {[index:string]:string});
    });
  }

  const onDoubleClickFileItem = (fileItem:FileItem) => {
    if(fileItem.file_type == "dir"){
      directoryHistory.push(directory);
      directory = fileItem.full_path;
    }else{
      invoke("open_file",{filePath:fileItem.full_path});
    }
  }

  const onClickHistoryBack = ()=>{
    if(directoryHistory.length == 0) return;
    directory = directoryHistory.pop()!;
  }

  const onClickChangeDir = (path:string) => {
    directory = path;
  }

</script>

<div class="container">
  <TitleBar />
  <div style="margin-top: 30px;"></div>
  <TopMenu bind:searchbarMode={searchbarMode} bind:directory={directory}
           onClickHistoryBack={onClickHistoryBack}
   />
  <div class="inner-container">
    <LeftItemList driveList={driveList} onClickChangeDir={onClickChangeDir} />
    <div class="file-list">
      <ListView fileItems={fileItems} iconCache={iconCache} onDoubleClickFileItem={onDoubleClickFileItem}/>
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
    height: 0;
    flex-grow: 1;
    display: flex;
  }
  .file-list{
    flex-grow: 1;
    overflow-y: hidden;
  }
</style>
