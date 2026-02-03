<template>
  <section class="app-main">
    <div class="pd10">
      <transition name="fade-transform" mode="out-in">
        <router-view/>
      </transition>
    </div>
  </section>
</template>

<script>
export default {
  name: 'AppMain',
  computed: {
    key() {
      return this.$route.path
    }
  }
}
</script>

<style scoped>
.app-main {
  width: 100%;
  position: relative;
  overflow-y: auto;
  height: calc(100vh - 60px);
}

.fixed-header + .app-main {
  padding-top: 50px;
}
</style>

<style lang="scss">
// fix css style bug in open el-dialog
.el-popup-parent--hidden {
  .fixed-header {
    padding-right: 15px;
  }
}
</style>
