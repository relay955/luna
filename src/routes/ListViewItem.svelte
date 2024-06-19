<script lang="ts">
    import type { FileItem } from "../logics/fileitem";
    import { beutifyFileSize } from "../utils/filesizeBeutifier";

    export let fileItem:FileItem;
</script>
<div class="list-view-item">
    <div>{fileItem.name}</div>
    <div>{fileItem.editDate}</div>
    <div>{beutifyFileSize(fileItem.size)}</div>
</div>

<style lang="scss">
    .list-view-item{
        display: flex;
        justify-content: space-between;
        padding: 10px;
        border-bottom: 1px solid #e8e8e8;
    }
</style>