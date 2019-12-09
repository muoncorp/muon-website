<template>
  <div>
    <v-img
      class="muon-header"
      src="../assets/header.png"
    />
    <h2>약도</h2>
    <Map />
    <h2>찾아 오시는 길</h2>
    <ul>
      <li>지하철 9호선 선유도역 8번 출구에서 도보로 3분 거리</li>
      <li>지하철 2,9호선 당산역 2번 출구 버스 정류장에서 양천01 마을버스 탑승후 한신아파트·선유도역 하차</li>
    </ul>
  </div>
</template>

<script>
  import { Component, Vue } from 'vue-property-decorator'
  import Map from '@/components/Map.vue'

  @Component({
    components: {
      Map,
    },
  })
  export default class Contact extends Vue {
  }
</script>

<style lang="scss">
.muon-header {
  width: 100%;
}
</style>
