<template>
  <div>
    <ImageHeader
      :src="require('../assets/about.jpg')"
      title="About"
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
    <v-fab-transition>
      <v-btn
        v-show="fab"
        v-scroll="onScroll"
        class="mb-10"
        color="primary lighten-3"
        fixed
        bottom
        right
        fab
        @click="toTop"
      >
        <v-icon>
          mdi-chevron-up
        </v-icon>
      </v-btn>
    </v-fab-transition>
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
    private fab = false

    mounted () {
      this.routeUpdated()
    }
    @Watch('$route')
    routeUpdated () {
      if (this.$route.hash) {
        this.$vuetify.goTo(this.$route.hash)
      }
    }
    onScroll (e: any) {
      if (typeof window === 'undefined') {
        return
      }
      const top = window.pageYOffset || e.target.scrollTop || 0
      this.fab = top > 20
    }
    toTop () {
      this.$vuetify.goTo(0)
    }
  }
</script>

<style lang="scss" scoped>
#overview {
  height: 300px;
}
#vision {
  height: 300px;
}
#history {
  height: 300px;
}
</style>
