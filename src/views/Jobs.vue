<template>
  <div>
    <ImageHeader
      :src="require('../assets/jobs.jpg')"
      title="Jobs"
    />
  </div>
</template>

<script lang="ts">
  import { Component, Vue, Watch } from 'vue-property-decorator'
  import ImageHeader from '@/components/ImageHeader.vue'

  @Component({
    components: {
      ImageHeader,
    },
  })
  export default class About extends Vue {
    mounted () {
      this.routeUpdated()
    }
    @Watch('$route')
    routeUpdated () {
      if (this.$route.hash) {
        this.$vuetify.goTo(this.$route.hash)
      }
    }
  }
</script>

<style lang="scss" scoped>
</style>
