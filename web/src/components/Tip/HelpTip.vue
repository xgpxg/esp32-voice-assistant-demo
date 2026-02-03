<template>
    <el-tooltip v-bind="$attrs">
      <template v-if="type==='warning'" #default>
        <svg-icon icon-class="warning" :size="14"></svg-icon>
      </template>
      <template v-else #default>
        <svg-icon icon-class="help" color="#aaaaaa" :size="14"></svg-icon>
      </template>
      <template #content>
        <div style="white-space: pre-line;width: 250px">
          <slot>
            {{ content }}
          </slot>
        </div>
      </template>
    </el-tooltip>
</template>

<script>
import SvgIcon from "../SvgIcon/index.vue";

export default {
  name: "HelpTip",
  components: {SvgIcon},
  props: {
    content: {
      type: String
    },
    type: String
  }
}
</script>

<style scoped>

</style>