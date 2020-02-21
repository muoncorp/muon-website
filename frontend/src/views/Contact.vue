<template>
  <div>
    <ImageHeader
      :src="require('../assets/contact.jpg')"
      title="Contact"
    />
    <h2 id="contact">
      Contact Us
    </h2>
    <h2 id="location">
      Location
    </h2>
    <Map
      :class="{'ma-4': $vuetify.breakpoint.mdAndUp}"
    />
    <h3>찾아 오시는 길</h3>
    <h4>지하철</h4>
    <ul>
      <li>지하철 9호선 선유도역 8번 출구에서 도보로 3분 거리</li>
    </ul>
    <h4>버스</h4>
    <ul>
      <li>지하철 2,9호선 당산역 2번 출구 버스 정류장에서 [양천01] 마을버스 탑승후 한신아파트·선유도역 하차</li>
    </ul>
  </div>
</template>

<script>
  import { Component, Vue } from 'vue-property-decorator'
  import ImageHeader from '@/components/ImageHeader.vue'
  import Map from '@/components/Map.vue'

  @Component({
    components: {
      ImageHeader,
      Map,
    },
  })
  export default class Contact extends Vue {
  }
</script>

<style lang="scss" scoped>
</style>
