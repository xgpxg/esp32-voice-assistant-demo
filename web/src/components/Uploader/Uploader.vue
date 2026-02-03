<script setup>

</script>

<template>
  <el-upload class="import-area"
             :auto-upload="false"
  >
    <div @click="">
      <el-text>将文件拖到此处，或点击导入</el-text>
    </div>
  </el-upload>

</template>

<style scoped lang="scss">
.import-area {
  width: 100%;
  text-align: center;
  line-height: 70px;
  color: #ccc;
  cursor: pointer;
  border-radius: 5px;
  border: 2px dashed #c0c4cc;

  &:hover {
    border-color: var(--el-color-primary);
    background-color: var(--el-color-primary-light-11);
  }
}
</style>