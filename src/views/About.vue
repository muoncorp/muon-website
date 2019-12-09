<template>
  <div>
    <v-img
      class="muon-header"
      src="../assets/overview.png"
    />
    <h2 id="overview">
      Overview
    </h2>
    <h2 id="vision">
      Vision
    </h2>
    <h2 id="history">
      History
    </h2>
  </div>
</template>

<script>
  import { Component, Vue } from 'vue-property-decorator'

  @Component({
    components: {
    },
  })
  export default class About extends Vue {
  }
</script>

<style lang="scss">
.muon-header {
  width: 100%;
}
</style>
