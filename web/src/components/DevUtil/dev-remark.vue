<template>
  <div v-if="U.isDev() && !hidden && showDevRemark">
    <div class="dev-remark">
      <slot></slot>
    </div>
  </div>
</template>

<script>
import defaultSettings from "@/settings";

export default {
  name: "dev-remark",
  components: {},
  props: {
    hidden: Boolean
  },
  data() {
    return {}
  },
  computed: {
    showDevRemark() {
      return defaultSettings.showDevRemark
    }
  },
  created() {
    this.init()
  },
  methods: {
    init() {

    }
  }
}
</script>


<style scoped lang="scss">
.dev-remark {
  border: #e5e5e5 1px solid;
  padding: 10px;
  background: #fff7da;
  line-height: 25px;
}
</style>